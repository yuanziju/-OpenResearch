/*
 * Copyright (c) 2023, 2026, Oracle and/or its affiliates. All rights reserved.
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
// Port of `jdk.graal.compiler.util.json.test.JsonParserTest`.

use crate::{JsonNumber, JsonParser, JsonValue};
use rustci_collections::{BTreeEconomicMap, UnmodifiableEconomicMap};
use std::io::Cursor;

// ── simpleJSON ───────────────────────────────────────────────────────────
const SIMPLE_JSON: &str = "{\"outer\": {\n\
    \"value\": \"test\",\n\
    \"inner\": {\n\
        \"title\": \"GraalVM\",\n\
        \"array\": [true, false, null],\n\
        \"version\": 42.5,\n\
        \"versionLong\": 9007199254740991,\n\
        \"size\": -1.2e2,\n\
        \"escapes\": \" \\\\ \\\" \\/ \\n \\b \\f \\r \\t \\u09aF \"\n\
    }\n\
}}";

// ── Helpers ──────────────────────────────────────────────────────────────

fn get_map<'a>(
    map: &'a BTreeEconomicMap<String, JsonValue>,
    key: &str,
) -> &'a BTreeEconomicMap<String, JsonValue> {
    let key_string = key.to_string();
    match map.get(&key_string).unwrap() {
        JsonValue::Object(m) => m,
        other => panic!("Expected object for key '{}', got {:?}", key, other),
    }
}

fn get_list<'a>(map: &'a BTreeEconomicMap<String, JsonValue>, key: &str) -> &'a Vec<JsonValue> {
    let key_string = key.to_string();
    match map.get(&key_string).unwrap() {
        JsonValue::Array(a) => a,
        other => panic!("Expected array for key '{}', got {:?}", key, other),
    }
}

fn test_simple_intl(parser: &mut JsonParser) {
    let result = parser.parse().unwrap();

    assert!(matches!(result, JsonValue::Object(_)));

    let map = match &result {
        JsonValue::Object(m) => m,
        _ => unreachable!(),
    };
    let outer = get_map(map, "outer");
    let inner = get_map(outer, "inner");

    // Java: Assert.assertEquals("test", outer.get("value"));
    assert_eq!(
        outer.get(&"value".to_string()),
        Some(&JsonValue::String("test".to_string()))
    );

    // Java: Assert.assertEquals("GraalVM", inner.get("title"));
    assert_eq!(
        inner.get(&"title".to_string()),
        Some(&JsonValue::String("GraalVM".to_string()))
    );

    // Java: List<Object> array = getList(inner, "array");
    let array = get_list(inner, "array");

    // Java: Assert.assertEquals(42.5, inner.get("version"));
    assert_eq!(
        inner.get(&"version".to_string()),
        Some(&JsonValue::Number(JsonNumber::Double(42.5)))
    );

    // Java: Assert.assertEquals(9007199254740991L, inner.get("versionLong"));
    assert_eq!(
        inner.get(&"versionLong".to_string()),
        Some(&JsonValue::Number(JsonNumber::Long(9007199254740991)))
    );

    // Java: Assert.assertEquals(-120.0, inner.get("size"));
    assert_eq!(
        inner.get(&"size".to_string()),
        Some(&JsonValue::Number(JsonNumber::Double(-120.0)))
    );

    // Java: Assert.assertEquals(" \\ \" / \n \b \f \r \t \u09AF ", inner.get("escapes"));
    assert_eq!(
        inner.get(&"escapes".to_string()),
        Some(&JsonValue::String(
            " \\ \" / \n \u{08} \u{0c} \r \t \u{09AF} ".to_string()
        ))
    );

    // Java: Assert.assertEquals(Boolean.TRUE, array.get(0));
    assert_eq!(array[0], JsonValue::Bool(true));
    // Java: Assert.assertEquals(Boolean.FALSE, array.get(1));
    assert_eq!(array[1], JsonValue::Bool(false));
    // Java: Assert.assertEquals(null, array.get(2));
    assert_eq!(array[2], JsonValue::Null);
}

// ── testSimpleJSONString ─────────────────────────────────────────────────
#[test]
fn test_simple_json_string() {
    // Java:
    //   JsonParser parser = new JsonParser(simpleJSON);
    //   testSimpleIntl(parser);
    let mut parser = JsonParser::new_from_str(SIMPLE_JSON);
    test_simple_intl(&mut parser);
}

// ── testSimpleJSONReader ─────────────────────────────────────────────────
#[test]
fn test_simple_json_reader() {
    // Java:
    //   JsonParser parser = new JsonParser(new StringReader(simpleJSON));
    //   testSimpleIntl(parser);
    let cursor = Cursor::new(SIMPLE_JSON.as_bytes());
    let mut parser = JsonParser::new_from_reader(cursor).unwrap();
    test_simple_intl(&mut parser);
}

// ── testErrors ───────────────────────────────────────────────────────────
#[test]
fn test_errors() {
    // Java: first test that simpleJSON parses OK
    let mut parser = JsonParser::new_from_str(SIMPLE_JSON);
    test_simple_intl(&mut parser);

    // Java: testErrorIntl calls
    test_error_intl("{ \"a\": \"\\uABCX\" }", "Invalid hex digit");
    test_error_intl("{ \"a\": trux }", "json literal");

    test_error_intl("{ \"a\": .0123}", "Invalid JSON number format");
    test_error_intl("{ \"a\": 1.2e-x}", "Invalid JSON number format");
    test_error_intl("{ \"a\": 1.x}", "Invalid JSON number format");
    test_error_intl("{ \"a\": -x}", "Invalid JSON number format");

    // Java: testErrorIntl("{ \"a\": \"" + (char) 0 + "\"}", "String contains control character");
    test_error_intl(
        &format!("{{ \"a\": \"{}\"}}", '\x00'),
        "String contains control character",
    );

    test_error_intl("{ \"a\": [", ", or ]");
    test_error_intl("{ \"a\": [,] }", "Trailing comma is not allowed in JSON");
    test_error_intl("{ \"a\": [1,] }", "Trailing comma is not allowed in JSON");
    test_error_intl("{ \"a\": [1 2] }", ", or ]");

    test_error_intl("{ \"a\" .0123}", "Expected :");

    test_error_intl("{ \"a\": \"\\v\"}", "Invalid escape character");

    test_error_intl("{ \"a\": \"string", "Missing close quote");

    test_error_intl("{ \"a\": true", ", or }");
    test_error_intl("{ \"a\": true \"b\" }", ", or }");

    test_error_intl("", "json literal");
    test_error_intl("keyword", "json literal");

    test_error_intl("{} something else", "eof");

    test_error_intl("{,}", "Trailing comma is not allowed in JSON");
    test_error_intl("{ \"a\": true, }", "Trailing comma is not allowed in JSON");
    test_error_intl("{ \"a\": true false}", ", or }");

    // offending token in new line; tests findBOLN
    test_error_intl("{ \"a\": true \n\nfalse}", ", or }");
}

fn test_error_intl(json: &str, expected_message: &str) {
    let mut parser = JsonParser::new_from_str(json);
    let result = parser.parse();
    match result {
        Ok(_) => panic!(
            "passed when a failure was expected. Expected error: {}",
            expected_message
        ),
        Err(ex) => {
            assert!(
                ex.to_string().contains(expected_message),
                "Expected error message to contain '{}', but got '{}'",
                expected_message,
                ex
            );
        }
    }
}

// ── parseAllowedKeysSimple ───────────────────────────────────────────────
#[test]
fn parse_allowed_keys_simple() {
    // Java:
    //   String source = " { \"foo\": 1, \"notFoo\": 2, \"bar\": 3 } ";
    //   JsonParser parser = new JsonParser(source);
    //   EconomicMap<String, Object> map = parser.parseAllowedKeys(List.of("foo", "bar", "baz"));
    //   Assert.assertEquals(2, map.size());
    //   Assert.assertEquals(1, map.get("foo"));
    //   Assert.assertEquals(3, map.get("bar"));
    let source = " { \"foo\": 1, \"notFoo\": 2, \"bar\": 3 } ";
    let mut parser = JsonParser::new_from_str(source);
    let allowed: Vec<String> = vec!["foo".into(), "bar".into(), "baz".into()];
    let map = parser.parse_allowed_keys(&allowed).unwrap();

    assert_eq!(map.size(), 2);
    assert_eq!(
        map.get(&"foo".to_string()),
        Some(&JsonValue::Number(JsonNumber::Int(1)))
    );
    assert_eq!(
        map.get(&"bar".to_string()),
        Some(&JsonValue::Number(JsonNumber::Int(3)))
    );
}

// ── parseAllowedKeysEarlyExit ────────────────────────────────────────────
#[test]
fn parse_allowed_keys_early_exit() {
    // Java:
    //   String source = "{\"foo\": 1, invalid syntax ";
    //   JsonParser parser = new JsonParser(source);
    //   EconomicMap<String, Object> map = parser.parseAllowedKeys(List.of("foo"));
    //   Assert.assertEquals(1, map.size());
    //   Assert.assertEquals(1, map.get("foo"));
    let source = "{\"foo\": 1, invalid syntax ";
    let mut parser = JsonParser::new_from_str(source);
    let allowed: Vec<String> = vec!["foo".into()];
    let map = parser.parse_allowed_keys(&allowed).unwrap();

    assert_eq!(map.size(), 1);
    assert_eq!(
        map.get(&"foo".to_string()),
        Some(&JsonValue::Number(JsonNumber::Int(1)))
    );
}

// ── parseAllowedKeysEmpty ────────────────────────────────────────────────
#[test]
fn parse_allowed_keys_empty() {
    // Java:
    //   Assert.assertTrue(new JsonParser("invalid syntax").parseAllowedKeys(List.of()).isEmpty());
    let mut parser = JsonParser::new_from_str("invalid syntax");
    let allowed: Vec<String> = vec![];
    let map = parser.parse_allowed_keys(&allowed).unwrap();
    assert!(map.size() == 0);
}

// ── parseAllowedKeysErrors ───────────────────────────────────────────────
#[test]
fn parse_allowed_keys_errors() {
    // Java:
    //   for (String source : List.of("", "[]", "{,}", "{\"a\": 1,}", "{\"a\": 1 \"")) {
    //       try {
    //           new JsonParser(source).parseAllowedKeys(List.of("foo"));
    //           Assert.fail("Should have failed to parse: " + source);
    //       } catch (JsonParserException ignored) {
    //       }
    //   }
    let sources = vec!["", "[]", "{,}", "{\"a\": 1,}", "{\"a\": 1 \""];
    let allowed: Vec<String> = vec!["foo".into()];
    for source in &sources {
        let mut parser = JsonParser::new_from_str(source);
        let result = parser.parse_allowed_keys(&allowed);
        assert!(result.is_err(), "Should have failed to parse: {}", source);
    }
}

// ── parserExceptionsAtEOF ────────────────────────────────────────────────
#[test]
fn parser_exceptions_at_eof() {
    // Java:
    //   for (String input : List.of("", "[", "{", "{\"", "{\"a", "{\"a\"", "{\"a\":", "[\"a\",", "tru", "nul", "fals")) {
    //       try {
    //           new JsonParser(input).parse();
    //           Assert.fail("The input string is not a valid JSON.");
    //       } catch (JsonParserException exception) {
    //           Assert.assertEquals(TriState.TRUE, exception.isAtEOF());
    //       }
    //   }
    let inputs = vec![
        "", "[", "{", "{\"", "{\"a", "{\"a\"", "{\"a\":", "[\"a\",", "tru", "nul", "fals",
    ];
    for input in &inputs {
        let mut parser = JsonParser::new_from_str(input);
        let result = parser.parse();
        match result {
            Ok(_) => panic!("The input string is not a valid JSON: {}", input),
            Err(exception) => {
                assert_eq!(
                    exception.is_at_eof(),
                    Some(true),
                    "Expected isAtEOF=TRUE for input: {}",
                    input
                );
            }
        }
    }
}

// ── parserExceptionsBeforeEOF ────────────────────────────────────────────
#[test]
fn parser_exceptions_before_eof() {
    // Java:
    //   for (String input : List.of("true??", "[?,", "{\"a\",\"b\"}")) {
    //       try {
    //           new JsonParser(input).parse();
    //           Assert.fail("The input string is not a valid JSON.");
    //       } catch (JsonParserException exception) {
    //           Assert.assertEquals(TriState.FALSE, exception.isAtEOF());
    //       }
    //   }
    let inputs = vec!["true??", "[?,", "{\"a\",\"b\"}"];
    for input in &inputs {
        let mut parser = JsonParser::new_from_str(input);
        let result = parser.parse();
        match result {
            Ok(_) => panic!("The input string is not a valid JSON: {}", input),
            Err(exception) => {
                assert_eq!(
                    exception.is_at_eof(),
                    Some(false),
                    "Expected isAtEOF=FALSE for input: {}",
                    input
                );
            }
        }
    }
}
