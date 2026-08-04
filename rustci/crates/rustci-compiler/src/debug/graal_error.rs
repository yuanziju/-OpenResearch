/*
 * Copyright (c) 2024, 2026, Oracle and/or its affiliates. All rights reserved.
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
// Rust mirror of `jdk.graal.compiler.debug.GraalError`.

use std::error::Error;
use std::fmt;

/// Mirrors `jdk.graal.compiler.debug.GraalError`.
/// Base error type for Graal compiler errors.
#[derive(Debug)]
pub struct GraalError {
    message: String,
    cause: Option<Box<dyn Error + Send + Sync>>,
    /// Whether the error should be printed with a stack trace.
    print_stack_trace: bool,
}

impl GraalError {
    /// Mirrors `GraalError(String)`.
    pub fn new(message: String) -> Self {
        GraalError {
            message,
            cause: None,
            print_stack_trace: true,
        }
    }

    /// Mirrors `GraalError(String, Throwable)`.
    pub fn with_cause(message: String, cause: Box<dyn Error + Send + Sync>) -> Self {
        GraalError {
            message,
            cause: Some(cause),
            print_stack_trace: true,
        }
    }

    /// Mirrors `GraalError.shouldPrintStackTrace()`.
    pub fn should_print_stack_trace(&self) -> bool {
        self.print_stack_trace
    }

    /// Mirrors the pattern of suppressing stack trace for certain errors.
    pub fn set_print_stack_trace(&mut self, print: bool) {
        self.print_stack_trace = print;
    }

    /// Mirrors `GraalError.getMessage()`.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Mirrors `GraalError.getCause()`.
    pub fn cause(&self) -> Option<&(dyn Error + Send + Sync)> {
        self.cause.as_deref()
    }

    /// Mirrors the static factory method `GraalError.shouldNotReachHere()`.
    pub fn should_not_reach_here() -> Self {
        GraalError::new("should not reach here".to_string())
    }

    /// Mirrors `GraalError.shouldNotReachHere(String)`.
    pub fn should_not_reach_here_msg(msg: &str) -> Self {
        GraalError::new(format!("should not reach here: {}", msg))
    }

    /// Mirrors `GraalError.shouldNotReachHere(Throwable)`.
    pub fn should_not_reach_here_cause(cause: Box<dyn Error + Send + Sync>) -> Self {
        GraalError::with_cause("should not reach here".to_string(), cause)
    }

    /// Mirrors `GraalError.unimplemented()`.
    pub fn unimplemented() -> Self {
        GraalError::new("unimplemented".to_string())
    }

    /// Mirrors `GraalError.unimplemented(String)`.
    pub fn unimplemented_msg(msg: &str) -> Self {
        GraalError::new(format!("unimplemented: {}", msg))
    }

    /// Mirrors `GraalError.guarantee(boolean, String, Object...)`.
    pub fn guarantee(condition: bool, msg: &str) -> Result<(), GraalError> {
        if !condition {
            Err(GraalError::new(format!("guarantee failed: {}", msg)))
        } else {
            Ok(())
        }
    }

    /// Mirrors `GraalError.unimplementedOverride()`.
    pub fn unimplemented_override() -> Self {
        GraalError::new("unimplemented override".to_string())
    }
}

impl fmt::Display for GraalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(ref cause) = self.cause {
            write!(f, "\nCaused by: {}", cause)?;
        }
        Ok(())
    }
}

impl Error for GraalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}