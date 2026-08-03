/*
 * Copyright (c) 2021, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 *
 * Subject to the condition set forth below, permission is hereby granted to any
 * person obtaining a copy of this software, associated documentation and/or
 * data (collectively the "Software"), free of charge and under any and all
 * copyright rights in the Software, and any and all patent rights owned or
 * freely licensable by each licensor hereunder covering either (i) the
 * unmodified Software as contributed to or provided by such licensor, or (ii)
 * the Larger Works (as defined below), to deal in both
 *
 * (a) the Software, and
 *
 * (b) any piece of software and/or hardware listed in the lrgrwrks.txt file if
 * one is included with the Software each a "Larger Work" to which the Software
 * is contributed by such licensors),
 *
 * without restriction, including without limitation the rights to copy, create
 * derivative works of, display, perform, and distribute the Software and make,
 * use, sell, offer for sale, import, export, have made, and have sold the
 * Software and the Larger Work(s), and to sublicense the foregoing rights on
 * either these or other terms.
 *
 * This license is subject to the following condition:
 *
 * The above copyright notice and either this complete permission notice or at a
 * minimum a reference to the UPL must be included in all copies or substantial
 * portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception
//
// Port of `jdk.graal.compiler.util.json.JsonParser`.
//
// Java uses `Reader` / `BufferedReader` with a `CharBuffer` (8192 chars).
// Rust reads the entire source into a `Vec<u8>` in the constructor, then
// uses an index cursor for byte-at-a-time parsing.  The parsing logic
// (state machine, escape handling, number parsing, keyword matching) is a
// 1:1 mirror of the Java implementation.

use crate::json_parser_exception::JsonParserException;
use crate::json_value::{JsonNumber, JsonValue};
use rustci_collections::{BTreeEconomicMap, EconomicMap, UnmodifiableEconomicMap};
use std::io::Read;

/// Mirrors `jdk.graal.compiler.util.json.JsonParser`.
pub struct JsonParser {
    /// Entire source content as bytes.
    source: Vec<u8>,
    /// Current reading position within `source` (1-based, mirroring Java's
    /// `pos` field that increments after each `next()` call).
    pos: usize,
    /// Current line number (0-based), used for error reporting.
    line: usize,
    /// Position of the start of the current line in `source`, used for error
    /// reporting.
    beginning_of_line: usize,
    /// Next byte to be scanned. `None` represents EOF (mirrors Java's `-1`).
    next: Option<u8>,
    /// Current index into `source`. The byte at `source[index]` is the
    /// *next* byte to be consumed by `next_byte()`.
    index: usize,
}

/// State constants mirroring Java's `STATE_EMPTY`, `STATE_ELEMENT_PARSED`,
/// `STATE_COMMA_PARSED`.
const STATE_EMPTY: u8 = 0;
const STATE_ELEMENT_PARSED: u8 = 1;
const STATE_COMMA_PARSED: u8 = 2;

impl JsonParser {
    /// Creates a new `JsonParser` to parse the given string. Mirrors
    /// `JsonParser(String)`.
    pub fn new_from_str(source: &str) -> Self {
        let mut parser = JsonParser {
            source: source.as_bytes().to_vec(),
            pos: 0,
            line: 0,
            beginning_of_line: 0,
            next: None,
            index: 0,
        };
        parser.next_byte();
        parser.pos = 0;
        parser
    }

    /// Creates a new `JsonParser` that reads all bytes from `source`. Mirrors
    /// `JsonParser(Reader)`.
    pub fn new_from_reader<R: Read>(mut source: R) -> std::io::Result<Self> {
        let mut buf = Vec::new();
        source.read_to_end(&mut buf)?;
        let mut parser = JsonParser {
            source: buf,
            pos: 0,
            line: 0,
            beginning_of_line: 0,
            next: None,
            index: 0,
        };
        parser.next_byte();
        parser.pos = 0;
        Ok(parser)
    }

    /// Returns the length of the source. Mirrors `getSourceLength()`.
    pub fn get_source_length(&self) -> usize {
        self.source.len()
    }

    /// Parses the next value from the underlying reader as a JSON value.
    /// Mirrors `parse()`.
    pub fn parse(&mut self) -> Result<JsonValue, JsonParserException> {
        let value = self.parse_literal()?;
        self.skip_white_space();
        if self.next.is_some() {
            return Err(self.expected_error(self.pos, "eof", &self.to_string(self.peek())));
        }
        Ok(value)
    }

    /// Parses the next value as a JSON object using a list of allowed keys.
    /// Mirrors `parseAllowedKeys(List<String>)`.
    pub fn parse_allowed_keys(
        &mut self,
        allowed_keys: &[String],
    ) -> Result<BTreeEconomicMap<String, JsonValue>, JsonParserException> {
        let mut result = BTreeEconomicMap::create();
        if allowed_keys.is_empty() {
            self.next = None;
            return Ok(result);
        }
        self.skip_white_space();
        let mut state = STATE_EMPTY;

        let c = self.peek();
        if c.is_none() {
            return Err(self.expected_error(self.pos, "json literal", "eof"));
        }
        if c.unwrap() != b'{' {
            return Err(self.expected_error(self.pos, "{", &self.to_string(c)));
        }
        self.next_byte();

        while self.next.is_some() {
            self.skip_white_space();
            let c = self.peek();

            match c {
                Some(b'"') => {
                    if state == STATE_ELEMENT_PARSED {
                        return Err(self.expected_error(self.pos, ", or }", &self.to_string(c)));
                    }
                    let id = self.parse_string()?;
                    self.expect_colon()?;
                    let value = self.parse_literal()?;
                    if allowed_keys.contains(&id) {
                        result.put(id, Some(value));
                    }
                    if result.size() == allowed_keys.len() {
                        self.next = None;
                        return Ok(result);
                    }
                    state = STATE_ELEMENT_PARSED;
                }
                Some(b',') => {
                    if state != STATE_ELEMENT_PARSED {
                        return Err(self.error("Trailing comma is not allowed in JSON", self.pos));
                    }
                    state = STATE_COMMA_PARSED;
                    self.next_byte();
                }
                Some(b'}') => {
                    if state == STATE_COMMA_PARSED {
                        return Err(self.error("Trailing comma is not allowed in JSON", self.pos));
                    }
                    self.next_byte();
                    return Ok(result);
                }
                _ => {
                    return Err(self.expected_error(self.pos, ", or }", &self.to_string(c)));
                }
            }
        }
        Err(self.expected_error(self.pos, ", or }", "eof"))
    }

    /// Utility method to parse a character stream containing a JSON object
    /// into an `EconomicMap` directly. Mirrors `parseDict(Reader)`.
    pub fn parse_dict_reader<R: Read>(
        input: R,
    ) -> Result<BTreeEconomicMap<String, JsonValue>, JsonParserException> {
        let mut parser = JsonParser::new_from_reader(input)
            .map_err(|e| JsonParserException::new(e.to_string()))?;
        match parser.parse()? {
            JsonValue::Object(map) => Ok(map),
            _ => Err(JsonParserException::new("Expected JSON object".to_string())),
        }
    }

    /// Utility method to parse a string containing a JSON object into an
    /// `EconomicMap` directly. Mirrors `parseDict(String)`.
    pub fn parse_dict_str(
        input: &str,
    ) -> Result<BTreeEconomicMap<String, JsonValue>, JsonParserException> {
        let mut parser = JsonParser::new_from_str(input);
        match parser.parse()? {
            JsonValue::Object(map) => Ok(map),
            _ => Err(JsonParserException::new("Expected JSON object".to_string())),
        }
    }

    // ── private methods ──────────────────────────────────────────────────

    /// Mirrors `parseLiteral()`.
    fn parse_literal(&mut self) -> Result<JsonValue, JsonParserException> {
        self.skip_white_space();

        let c = self.peek();
        if c.is_none() {
            return Err(self.expected_error(self.pos, "json literal", "eof"));
        }
        match c.unwrap() {
            b'{' => self.parse_object(),
            b'[' => self.parse_array(),
            b'"' => self.parse_string().map(JsonValue::String),
            b'f' => self.parse_keyword("false", JsonValue::Bool(false)),
            b't' => self.parse_keyword("true", JsonValue::Bool(true)),
            b'n' => self.parse_keyword("null", JsonValue::Null),
            _ => {
                if Self::is_digit(c.unwrap()) || c.unwrap() == b'-' {
                    self.parse_number()
                } else if c.unwrap() == b'.' {
                    Err(self.number_error(self.pos))
                } else {
                    Err(self.expected_error(self.pos, "json literal", &self.to_string(c)))
                }
            }
        }
    }

    /// Mirrors `parseObject()`.
    fn parse_object(&mut self) -> Result<JsonValue, JsonParserException> {
        let mut result = BTreeEconomicMap::create();
        let mut state = STATE_EMPTY;

        let p = self.peek();
        debug_assert_eq!(p, Some(b'{'), "Must be {{ but was {:?}", p);
        self.next_byte();

        while self.next.is_some() {
            self.skip_white_space();
            let c = self.peek();

            match c {
                Some(b'"') => {
                    if state == STATE_ELEMENT_PARSED {
                        return Err(self.expected_error(self.pos, ", or }", &self.to_string(c)));
                    }
                    let id = self.parse_string()?;
                    self.expect_colon()?;
                    let value = self.parse_literal()?;
                    result.put(id, Some(value));
                    state = STATE_ELEMENT_PARSED;
                }
                Some(b',') => {
                    if state != STATE_ELEMENT_PARSED {
                        return Err(self.error("Trailing comma is not allowed in JSON", self.pos));
                    }
                    state = STATE_COMMA_PARSED;
                    self.next_byte();
                }
                Some(b'}') => {
                    if state == STATE_COMMA_PARSED {
                        return Err(self.error("Trailing comma is not allowed in JSON", self.pos));
                    }
                    self.next_byte();
                    return Ok(JsonValue::Object(result));
                }
                _ => {
                    return Err(self.expected_error(self.pos, ", or }", &self.to_string(c)));
                }
            }
        }
        Err(self.expected_error(self.pos, ", or }", "eof"))
    }

    /// Mirrors `expectColon()`.
    fn expect_colon(&mut self) -> Result<(), JsonParserException> {
        self.skip_white_space();
        let n = self.next_byte();
        if n != Some(b':') {
            return Err(self.expected_error(self.pos.saturating_sub(1), ":", &self.to_string(n)));
        }
        Ok(())
    }

    /// Mirrors `parseArray()`.
    fn parse_array(&mut self) -> Result<JsonValue, JsonParserException> {
        let mut result: Vec<JsonValue> = Vec::new();
        let mut state = STATE_EMPTY;

        let p = self.peek();
        debug_assert_eq!(p, Some(b'['), "Must be [ but was {:?}", p);
        self.next_byte();

        while self.next.is_some() {
            self.skip_white_space();
            let c = self.peek();

            match c {
                Some(b',') => {
                    if state != STATE_ELEMENT_PARSED {
                        return Err(self.error("Trailing comma is not allowed in JSON", self.pos));
                    }
                    state = STATE_COMMA_PARSED;
                    self.next_byte();
                }
                Some(b']') => {
                    if state == STATE_COMMA_PARSED {
                        return Err(self.error("Trailing comma is not allowed in JSON", self.pos));
                    }
                    self.next_byte();
                    return Ok(JsonValue::Array(result));
                }
                _ => {
                    if state == STATE_ELEMENT_PARSED {
                        return Err(self.expected_error(self.pos, ", or ]", &self.to_string(c)));
                    }
                    result.push(self.parse_literal()?);
                    state = STATE_ELEMENT_PARSED;
                }
            }
        }

        Err(self.expected_error(self.pos, ", or ]", "eof"))
    }

    /// Mirrors `parseString()`.
    fn parse_string(&mut self) -> Result<String, JsonParserException> {
        // Consume the opening quote.
        self.next_byte();
        // String buffer is only instantiated if string contains escape sequences.
        let mut sb: Vec<u8> = Vec::new();

        while self.next.is_some() {
            let c = self.next_byte().unwrap();
            if c <= 0x1f {
                // Characters <= 0x1f are not allowed in JSON strings.
                return Err(self.syntax_error(
                    self.pos,
                    &format!("String contains control character: {}", c),
                ));
            } else if c == b'\\' {
                let escaped = self.parse_escape_sequence()?;
                // Encode the escaped char as UTF-8.
                let mut buf = [0u8; 4];
                let encoded = escaped.encode_utf8(&mut buf);
                sb.extend_from_slice(encoded.as_bytes());
            } else if c == b'"' {
                return String::from_utf8(sb)
                    .map_err(|e| self.syntax_error(self.pos, &e.to_string()));
            } else {
                sb.push(c);
            }
        }

        Err(self.error("Missing close quote", self.pos))
    }

    /// Mirrors `parseEscapeSequence()`.
    fn parse_escape_sequence(&mut self) -> Result<char, JsonParserException> {
        let c = self.next_byte();
        match c {
            Some(b'"') => Ok('"'),
            Some(b'\\') => Ok('\\'),
            Some(b'/') => Ok('/'),
            Some(b'b') => Ok('\x08'),
            Some(b'f') => Ok('\x0c'),
            Some(b'n') => Ok('\n'),
            Some(b'r') => Ok('\r'),
            Some(b't') => Ok('\t'),
            Some(b'u') => self.parse_unicode_escape(),
            _ => Err(self.error("Invalid escape character", self.pos.saturating_sub(1))),
        }
    }

    /// Mirrors `parseUnicodeEscape()`.
    fn parse_unicode_escape(&mut self) -> Result<char, JsonParserException> {
        let code = (self.parse_hex_digit()?) << 12
            | (self.parse_hex_digit()?) << 8
            | (self.parse_hex_digit()?) << 4
            | self.parse_hex_digit()?;
        char::from_u32(code)
            .ok_or_else(|| self.error("Invalid unicode escape", self.pos.saturating_sub(1)))
    }

    /// Mirrors `parseHexDigit()`.
    fn parse_hex_digit(&mut self) -> Result<u32, JsonParserException> {
        let c = self.next_byte();
        match c {
            Some(b'0'..=b'9') => Ok((c.unwrap() - b'0') as u32),
            Some(b'A'..=b'F') => Ok((c.unwrap() - b'A' + 10) as u32),
            Some(b'a'..=b'f') => Ok((c.unwrap() - b'a' + 10) as u32),
            _ => Err(self.error("Invalid hex digit", self.pos.saturating_sub(1))),
        }
    }

    /// Mirrors `isDigit(int)`.
    fn is_digit(c: u8) -> bool {
        c.is_ascii_digit()
    }

    /// Mirrors `skipDigits(StringBuilder)`.
    fn skip_digits(&mut self, sb: &mut String) {
        while self.next.is_some() {
            let c = self.peek().unwrap();
            if !Self::is_digit(c) {
                break;
            }
            self.next_byte();
            sb.push(c as char);
        }
    }

    /// Mirrors `parseNumber()`.
    fn parse_number(&mut self) -> Result<JsonValue, JsonParserException> {
        let start = self.pos;
        let mut sb = String::new();
        let mut c = self.next_byte().unwrap();
        sb.push(c as char);

        if c == b'-' {
            c = self.next_byte().unwrap();
            sb.push(c as char);
        }
        if !Self::is_digit(c) {
            return Err(self.number_error(start));
        }
        // no more digits allowed after 0
        if c != b'0' {
            self.skip_digits(&mut sb);
        }

        // fraction
        let mut is_floating = false;
        if self.peek() == Some(b'.') {
            is_floating = true;
            let ch = self.next_byte().unwrap();
            sb.push(ch as char);
            let ch = self.next_byte().unwrap();
            sb.push(ch as char);
            if !Self::is_digit(ch) {
                return Err(self.number_error(self.pos.saturating_sub(1)));
            }
            self.skip_digits(&mut sb);
        }

        // exponent
        c = self.peek().unwrap_or(0);
        if c == b'e' || c == b'E' {
            self.next_byte();
            sb.push(c as char);
            c = self.next_byte().unwrap();
            sb.push(c as char);
            if c == b'-' || c == b'+' {
                c = self.next_byte().unwrap();
                sb.push(c as char);
            }
            if !Self::is_digit(c) {
                return Err(self.number_error(self.pos.saturating_sub(1)));
            }
            self.skip_digits(&mut sb);
        }

        if is_floating {
            let d: f64 = sb.parse().map_err(|_| self.number_error(start))?;
            Ok(JsonValue::Number(JsonNumber::Double(d)))
        } else {
            let l: i64 = sb.parse().map_err(|_| self.number_error(start))?;
            if (l as i32) as i64 == l {
                Ok(JsonValue::Number(JsonNumber::Int(l as i32)))
            } else {
                Ok(JsonValue::Number(JsonNumber::Long(l)))
            }
        }
    }

    /// Mirrors `parseKeyword(String, Object)`.
    fn parse_keyword(
        &mut self,
        keyword: &str,
        value: JsonValue,
    ) -> Result<JsonValue, JsonParserException> {
        for expected_byte in keyword.bytes() {
            if self.next_byte() != Some(expected_byte) {
                return Err(self.expected_error(self.pos, "json literal", "ident"));
            }
        }
        Ok(value)
    }

    /// Mirrors `peek()`.
    fn peek(&self) -> Option<u8> {
        self.next
    }

    /// Mirrors `next()`. Advances to the next byte and returns the previous
    /// `next` value.
    fn next_byte(&mut self) -> Option<u8> {
        let cur = self.next;
        if self.index >= self.source.len() {
            self.next = None;
            return cur;
        }
        self.next = Some(self.source[self.index]);
        self.index += 1;
        self.pos += 1;
        cur
    }

    /// Mirrors `skipWhiteSpace()`.
    fn skip_white_space(&mut self) {
        while self.next.is_some() {
            match self.peek().unwrap() {
                b'\n' => {
                    self.line += 1;
                    self.beginning_of_line = self.pos + 1;
                    self.next_byte();
                }
                b'\r' => {
                    self.beginning_of_line = self.pos + 1;
                    self.next_byte();
                }
                b'\t' | b' ' => {
                    self.next_byte();
                }
                _ => return,
            }
        }
    }

    /// Mirrors `toString(int)`.
    fn to_string(&self, c: Option<u8>) -> String {
        match c {
            Some(b) => String::from(b as char),
            None => "eof".to_string(),
        }
    }

    /// Mirrors `error(String, int)`.
    fn error(&self, message: &str, position: usize) -> JsonParserException {
        let column_num = position.saturating_sub(self.beginning_of_line);
        let formatted = Self::format(message, self.line, column_num);
        JsonParserException::new_with_eof(formatted, self.peek().is_none())
    }

    /// Mirrors `format(String, int, int)`.
    fn format(message: &str, line: usize, column: usize) -> String {
        format!("line {} column {} {}", line, column, message)
    }

    /// Mirrors `numberError(int)`.
    fn number_error(&self, start: usize) -> JsonParserException {
        self.error("Invalid JSON number format", start)
    }

    /// Mirrors `expectedError(int, String, String)`.
    fn expected_error(&self, start: usize, expected: &str, found: &str) -> JsonParserException {
        self.error(&format!("Expected {} but found {}", expected, found), start)
    }

    /// Mirrors `syntaxError(int, String)`.
    fn syntax_error(&self, start: usize, reason: &str) -> JsonParserException {
        self.error(&format!("Invalid JSON: {}", reason), start)
    }
}
