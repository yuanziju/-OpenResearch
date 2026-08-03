/*
 * Copyright (c) 2017, 2026, Oracle and/or its affiliates. All rights reserved.
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
// Port of `org.graalvm.compiler.util.json.JsonWriter`.

use std::io;

use crate::json_printable::JsonPrintable;
use crate::json_printer::JsonPrinter;
use crate::json_value::{JsonNumber, JsonValue};
use crate::json_writer_trait::JsonWriter as JsonWriterTrait;
use rustci_collections::UnmodifiableEconomicMap;

// ---------------------------------------------------------------------------
// JsonWrite trait — the polymorphic interface that both `JsonWriter` and
// `JsonPrettyWriter` implement.  In Java the subtyping is achieved via
// inheritance; in Rust we use a trait with default method bodies so that the
// higher-level JSON-printing logic (quote, separator, printObject, printArray,
// etc.) is written once and the primitive emission hooks are provided by each
// concrete implementation.
// ---------------------------------------------------------------------------

/// Polymorphic write interface for JSON output. Mirrors the public API of
/// `jdk.graal.compiler.util.json.JsonWriter` so that `JsonPrintable` and
/// `JsonPrinter` can target both compact and pretty-printing writers.
pub trait JsonWrite {
    // -- primitive hooks (each concrete type implements these) --

    /// Write a single character to the underlying output.
    fn raw_emit_char(&mut self, c: char) -> io::Result<()>;

    /// Write a string slice to the underlying output.
    fn raw_emit_str(&mut self, s: &str) -> io::Result<()>;

    /// Whether property names should be quoted (default `true`).
    fn quoting_enabled(&self) -> bool;

    /// Read the current separator state.
    fn sep_state(&self) -> bool;

    /// Set the separator state.
    fn set_sep_state(&mut self, state: bool);

    /// Hook called when beginning an object.
    fn begin_object(&mut self) -> io::Result<()>;

    /// Hook called when ending an object.
    fn end_object(&mut self) -> io::Result<()>;

    /// Hook called when beginning an array.
    fn begin_array(&mut self) -> io::Result<()>;

    /// Hook called when ending an array.
    fn end_array(&mut self) -> io::Result<()>;

    // -- provided methods (shared logic) --

    /// Append a single character. Mirrors `JsonWriter append(char c)`.
    fn append_char(&mut self, c: char) -> io::Result<()> {
        self.raw_emit_char(c)
    }

    /// Append a string. Mirrors `JsonWriter append(String s)`.
    fn append_str(&mut self, s: &str) -> io::Result<()> {
        self.raw_emit_str(s)
    }

    /// Emit a comma separator if not the first element in the current
    /// object/array, then mark that a separator is needed for the next
    /// element. Mirrors `protected void separator()`.
    fn separator(&mut self) -> io::Result<()> {
        if self.sep_state() {
            self.raw_emit_char(',')?;
        }
        self.set_sep_state(true);
        Ok(())
    }

    /// Quote and escape a string. If `quoting_enabled()` is true the string is
    /// wrapped in double-quotes; otherwise it is emitted raw. Mirrors
    /// `JsonWriter quote(String s)`.
    fn quote(&mut self, s: &str) -> io::Result<()> {
        if self.quoting_enabled() {
            self.raw_emit_char('"')?;
        }
        for c in s.chars() {
            match c {
                '"' => self.raw_emit_str("\\\"")?,
                '\\' => self.raw_emit_str("\\\\")?,
                '\x08' => self.raw_emit_str("\\b")?,
                '\x0c' => self.raw_emit_str("\\f")?,
                '\n' => self.raw_emit_str("\\n")?,
                '\r' => self.raw_emit_str("\\r")?,
                '\t' => self.raw_emit_str("\\t")?,
                c if c < ' ' => {
                    let escaped = format!("\\u{:04x}", c as u32);
                    self.raw_emit_str(&escaped)?;
                }
                c => self.raw_emit_char(c)?,
            }
        }
        if self.quoting_enabled() {
            self.raw_emit_char('"')?;
        }
        Ok(())
    }

    /// Print a JSON object. The closure `f` receives `self` and writes the
    /// object body (key-value pairs). Mirrors
    /// `void printObject(Runnable r)`.
    fn print_object(
        &mut self,
        f: &mut dyn FnMut(&mut dyn JsonWrite) -> io::Result<()>,
    ) -> io::Result<()>
    where
        Self: Sized,
    {
        self.begin_object()?;
        self.set_sep_state(false);
        f(self)?;
        self.end_object()?;
        self.set_sep_state(true);
        Ok(())
    }

    /// Print a JSON array. The closure `f` receives `self` and writes the
    /// array elements. Mirrors `void printArray(Runnable r)`.
    fn print_array(
        &mut self,
        f: &mut dyn FnMut(&mut dyn JsonWrite) -> io::Result<()>,
    ) -> io::Result<()>
    where
        Self: Sized,
    {
        self.begin_array()?;
        self.set_sep_state(false);
        f(self)?;
        self.end_array()?;
        self.set_sep_state(true);
        Ok(())
    }

    /// Print a `JsonPrintable`. Mirrors `void print(JsonPrintable p)`.
    fn print_json_printable(&mut self, p: &dyn JsonPrintable) -> io::Result<()>
    where
        Self: Sized,
    {
        p.append_to(self)
    }
}

/// Print an object using a `JsonPrinter`. Mirrors
/// `<T> void print(JsonPrinter<T> printer, T obj)` on `JsonWriter`.
/// Defined as a free function because the generic `<T>` would make the
/// `JsonWrite` trait not dyn-compatible.
pub fn print_with_printer<T>(
    w: &mut dyn JsonWrite,
    printer: &dyn JsonPrinter<T>,
    obj: &T,
) -> io::Result<()> {
    printer.print(w, obj)
}

// ---------------------------------------------------------------------------
// JsonWriter — compact JSON writer (no indentation)
// ---------------------------------------------------------------------------

/// A compact JSON writer. Mirrors
/// `jdk.graal.compiler.util.json.JsonWriter`.
pub struct JsonWriter {
    out: Box<dyn io::Write>,
    quote: bool,
    separator: bool,
}

impl JsonWriter {
    /// Creates a new writer that writes to `out`. Mirrors
    /// `JsonWriter(Writer out)`.
    pub fn new(out: Box<dyn io::Write>) -> Self {
        JsonWriter {
            out,
            quote: true,
            separator: false,
        }
    }
}

impl JsonWrite for JsonWriter {
    fn raw_emit_char(&mut self, c: char) -> io::Result<()> {
        let mut buf = [0u8; 4];
        let s = c.encode_utf8(&mut buf);
        self.out.write_all(s.as_bytes())
    }

    fn raw_emit_str(&mut self, s: &str) -> io::Result<()> {
        self.out.write_all(s.as_bytes())
    }

    fn quoting_enabled(&self) -> bool {
        self.quote
    }

    fn sep_state(&self) -> bool {
        self.separator
    }

    fn set_sep_state(&mut self, state: bool) {
        self.separator = state;
    }

    fn begin_object(&mut self) -> io::Result<()> {
        self.raw_emit_char('{')
    }

    fn end_object(&mut self) -> io::Result<()> {
        self.raw_emit_char('}')
    }

    fn begin_array(&mut self) -> io::Result<()> {
        self.raw_emit_char('[')
    }

    fn end_array(&mut self) -> io::Result<()> {
        self.raw_emit_char(']')
    }
}

impl JsonWriterTrait for JsonWriter {
    fn append_object_start(&mut self) -> io::Result<()> {
        self.raw_emit_char('{')
    }

    fn append_object_end(&mut self) -> io::Result<()> {
        self.raw_emit_char('}')
    }

    fn append_array_start(&mut self) -> io::Result<()> {
        self.raw_emit_char('[')
    }

    fn append_array_end(&mut self) -> io::Result<()> {
        self.raw_emit_char(']')
    }

    fn append_separator(&mut self) -> io::Result<()> {
        self.raw_emit_char(',')
    }

    fn append_field_separator(&mut self) -> io::Result<()> {
        self.raw_emit_char(':')
    }

    fn print(&mut self, value: &JsonValue) -> io::Result<()> {
        match value {
            JsonValue::Null => self.raw_emit_str("null"),
            JsonValue::Bool(true) => self.raw_emit_str("true"),
            JsonValue::Bool(false) => self.raw_emit_str("false"),
            JsonValue::String(s) => JsonWrite::quote(self, s),
            JsonValue::Number(n) => match n {
                JsonNumber::Int(i) => self.raw_emit_str(&i.to_string()),
                JsonNumber::Long(l) => self.raw_emit_str(&l.to_string()),
                JsonNumber::Double(d) => {
                    if d.is_nan() || d.is_infinite() {
                        self.raw_emit_str("null")
                    } else {
                        self.raw_emit_str(&d.to_string())
                    }
                }
            },
            JsonValue::Array(arr) => {
                self.append_array_start()?;
                let mut first = true;
                for v in arr {
                    if !first {
                        self.append_separator()?;
                    }
                    first = false;
                    self.print(v)?;
                }
                self.append_array_end()
            }
            JsonValue::Object(map) => {
                self.append_object_start()?;
                let mut cursor = map.get_entries();
                let mut first = true;
                while cursor.advance() {
                    if !first {
                        self.append_separator()?;
                    }
                    first = false;
                    JsonWrite::quote(self, cursor.get_key())?;
                    self.append_field_separator()?;
                    if let Some(v) = cursor.get_value() {
                        self.print(v)?;
                    } else {
                        self.raw_emit_str("null")?;
                    }
                }
                self.append_object_end()
            }
        }
    }

    fn append_raw(&mut self, data: &[u8]) -> io::Result<()> {
        self.out.write_all(data)
    }

    fn append_quoted_string(&mut self, s: &str) -> io::Result<()> {
        for c in s.chars() {
            match c {
                '"' => self.raw_emit_str("\\\"")?,
                '\\' => self.raw_emit_str("\\\\")?,
                '\x08' => self.raw_emit_str("\\b")?,
                '\x0c' => self.raw_emit_str("\\f")?,
                '\n' => self.raw_emit_str("\\n")?,
                '\r' => self.raw_emit_str("\\r")?,
                '\t' => self.raw_emit_str("\\t")?,
                c if c < ' ' => {
                    let escaped = format!("\\u{:04x}", c as u32);
                    self.raw_emit_str(&escaped)?;
                }
                c => self.raw_emit_char(c)?,
            }
        }
        Ok(())
    }
}
