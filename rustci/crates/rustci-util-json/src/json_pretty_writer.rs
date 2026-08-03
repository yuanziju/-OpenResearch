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
// Port of `org.graalvm.compiler.util.json.JsonPrettyWriter`.

use std::io;

use crate::json_writer::JsonWrite;

// ---------------------------------------------------------------------------
// JsonPrettyWriter — indented / pretty-printing JSON writer
// ---------------------------------------------------------------------------

/// A pretty-printing JSON writer that adds indentation. Mirrors
/// `jdk.graal.compiler.util.json.JsonPrettyWriter` which extends
/// `JsonWriter` and overrides `append`, `printObject`, and `printArray`.
pub struct JsonPrettyWriter {
    out: Box<dyn io::Write>,
    quote: bool,
    separator: bool,
    /// Current indentation level (number of spaces).
    indent: usize,
}

impl JsonPrettyWriter {
    /// Creates a new pretty-printing writer that writes to `out`.
    pub fn new(out: Box<dyn io::Write>) -> Self {
        JsonPrettyWriter {
            out,
            quote: true,
            separator: false,
            indent: 0,
        }
    }
}

impl JsonWrite for JsonPrettyWriter {
    fn raw_emit_char(&mut self, c: char) -> io::Result<()> {
        // Track indent depth on structural characters, matching Java's
        // `append(char c)` override.
        if c == '[' || c == '{' {
            self.indent = self.indent.saturating_add(2);
        }
        if c == ']' || c == '}' {
            self.indent = self.indent.saturating_sub(2);
        }
        let mut buf = [0u8; 4];
        let s = c.encode_utf8(&mut buf);
        self.out.write_all(s.as_bytes())?;
        // After a newline, emit indentation spaces.
        if c == '\n' {
            for _ in 0..self.indent {
                self.out.write_all(b" ")?;
            }
        }
        Ok(())
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

    /// Override: emit a newline before `{`, then `{` (which increments indent).
    fn begin_object(&mut self) -> io::Result<()> {
        self.raw_emit_char('\n')?;
        self.raw_emit_char('{')
    }

    /// Override: emit a newline before `}`, then `}` (which decrements indent).
    fn end_object(&mut self) -> io::Result<()> {
        self.raw_emit_char('\n')?;
        self.raw_emit_char('}')
    }

    /// Override: emit a newline before `[`, then `[` (which increments indent).
    fn begin_array(&mut self) -> io::Result<()> {
        self.raw_emit_char('\n')?;
        self.raw_emit_char('[')
    }

    /// Override: emit a newline before `]`, then `]` (which decrements indent).
    fn end_array(&mut self) -> io::Result<()> {
        self.raw_emit_char('\n')?;
        self.raw_emit_char(']')
    }
}
