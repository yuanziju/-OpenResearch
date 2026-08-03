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
// Port of `jdk.graal.compiler.util.json.JsonParserException`.
//
// Java `JsonParserException` extends `RuntimeException` and carries a
// `TriState` `isAtEOF` field. Rust mirrors this with `Option<bool>` where
// `None` = TriState.UNKNOWN, `Some(true)` = TriState.TRUE,
// `Some(false)` = TriState.FALSE.

use std::error::Error;
use std::fmt;

/// Thrown by `JsonParser` if an error is encountered during parsing. Mirrors
/// `jdk.graal.compiler.util.json.JsonParserException`.
#[derive(Debug)]
pub struct JsonParserException {
    message: String,
    /// Whether the parser was at the end of the input when the exception
    /// occurred. `None` mirrors `TriState.UNKNOWN` for exceptions not thrown
    /// by `JsonParser`.
    is_at_eof: Option<bool>,
}

impl JsonParserException {
    /// Constructs a new JSON parser exception with the specified detail
    /// message. The state of whether the parser was at the end of the input
    /// when the exception occurred is unknown. Mirrors
    /// `JsonParserException(String)`.
    pub fn new(msg: String) -> Self {
        JsonParserException {
            message: msg,
            is_at_eof: None,
        }
    }

    /// Constructs a new JSON parser exception with the specified detail
    /// message and information about whether the parser was at the end of the
    /// input when the exception occurred. Mirrors
    /// `JsonParserException(String, boolean)`.
    pub fn new_with_eof(msg: String, is_at_eof: bool) -> Self {
        JsonParserException {
            message: msg,
            is_at_eof: Some(is_at_eof),
        }
    }

    /// Returns whether the parser was at the end of the input when the
    /// exception occurred. `None` mirrors `TriState.UNKNOWN` for exceptions
    /// that were not thrown by `JsonParser`. Mirrors `isAtEOF()`.
    pub fn is_at_eof(&self) -> Option<bool> {
        self.is_at_eof
    }
}

impl fmt::Display for JsonParserException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for JsonParserException {}
