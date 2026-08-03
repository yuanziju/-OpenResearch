/*
 * Copyright (c) 2024, 2026, Oracle and/or its affiliates. All rights reserved.
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
// Trait mirroring `jdk.graal.compiler.util.json.JsonWriter`'s interface
// surface. Defined as a trait so that `JsonBuilder` can compile independently
// of the concrete `JsonWriter` struct (which is being ported by a parallel
// worker). The concrete `JsonWriter` will implement this trait.

use crate::json_value::JsonValue;
use std::io;

/// Trait mirroring the methods of `jdk.graal.compiler.util.json.JsonWriter`
/// that `JsonBuilder` calls. A concrete `JsonWriter` struct (ported
/// separately) implements this trait.
pub trait JsonWriter {
    /// Writes the start of a JSON object (`{`). Mirrors
    /// `JsonWriter.appendObjectStart()`.
    fn append_object_start(&mut self) -> io::Result<()>;

    /// Writes the end of a JSON object (`}`). Mirrors
    /// `JsonWriter.appendObjectEnd()`.
    fn append_object_end(&mut self) -> io::Result<()>;

    /// Writes the start of a JSON array (`[`). Mirrors
    /// `JsonWriter.appendArrayStart()`.
    fn append_array_start(&mut self) -> io::Result<()>;

    /// Writes the end of a JSON array (`]`). Mirrors
    /// `JsonWriter.appendArrayEnd()`.
    fn append_array_end(&mut self) -> io::Result<()>;

    /// Writes an element separator (`,`). Mirrors
    /// `JsonWriter.appendSeparator()`.
    fn append_separator(&mut self) -> io::Result<()>;

    /// Writes a key-value separator (`:`). Mirrors
    /// `JsonWriter.appendFieldSeparator()`.
    fn append_field_separator(&mut self) -> io::Result<()>;

    /// Writes a quoted string. Mirrors `JsonWriter.quote(String)` returning
    /// `this` for chaining.  Returns `io::Result<()>` to keep the trait
    /// dyn-compatible; chaining callers call `quote` then the next method
    /// separately.
    fn quote(&mut self, key: &str) -> io::Result<()> {
        self.append_raw(b"\"")?;
        self.append_quoted_string(key)?;
        self.append_raw(b"\"")?;
        Ok(())
    }

    /// Writes a JSON value. Mirrors `JsonWriter.print(Object)`.
    fn print(&mut self, value: &JsonValue) -> io::Result<()>;

    /// Writes raw bytes to the output. Low-level primitive used by the
    /// default `quote` implementation.
    fn append_raw(&mut self, data: &[u8]) -> io::Result<()>;

    /// Writes the content of a string with proper JSON escaping (without
    /// surrounding quotes). Called by the default `quote` implementation.
    fn append_quoted_string(&mut self, s: &str) -> io::Result<()>;
}
