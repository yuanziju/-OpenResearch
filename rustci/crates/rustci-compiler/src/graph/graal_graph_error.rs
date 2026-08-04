/*
 * Copyright (c) 2011, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.  Oracle designates this
 * particular file as subject to the "Classpath" exception as provided
 * by Oracle in the LICENSE file that accompanied this code.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy is included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */

// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
//
// Rust mirror of `jdk.graal.compiler.graph.GraalGraphError`. Faithful 1:1 port.

use std::error::Error;
use std::fmt;

/// Corresponds to `public class GraalGraphError extends GraalError`.
///
/// Error type used for graph invariants and verification failures.
#[derive(Debug)]
pub struct GraalGraphError {
    message: String,
    cause: Option<Box<dyn Error>>,
}

impl GraalGraphError {
    /// Corresponds to `GraalGraphError(String msg)`.
    pub fn new(msg: String) -> Self {
        GraalGraphError {
            message: msg,
            cause: None,
        }
    }

    /// Corresponds to `GraalGraphError(Throwable cause, String msg)`.
    pub fn with_cause(cause: Box<dyn Error>, msg: String) -> Self {
        GraalGraphError {
            message: msg,
            cause: Some(cause),
        }
    }

    /// Returns the cause of this error, if any.
    pub fn cause(&self) -> Option<&dyn Error> {
        self.cause.as_ref().map(|c| c.as_ref())
    }
}

impl fmt::Display for GraalGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref cause) = self.cause {
            write!(f, "{}: {}", self.message, cause)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl Error for GraalGraphError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_ref().map(|c| c.as_ref() as &(dyn Error + 'static))
    }
}