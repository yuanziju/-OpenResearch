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
// Rust mirror of `jdk.graal.compiler.debug.Assertions`.

/// Mirrors `jdk.graal.compiler.debug.Assertions`.
/// Provides assertion utilities for Graal compiler debugging.
pub struct Assertions;

impl Assertions {
    /// Mirrors `Assertions.assertTrue(boolean, String, Object...)`.
    /// Throws a panic if the condition is false.
    pub fn assert_true(condition: bool, msg: &str) {
        assert!(condition, "{}", msg);
    }

    /// Mirrors `Assertions.assertFalse(boolean, String, Object...)`.
    pub fn assert_false(condition: bool, msg: &str) {
        assert!(!condition, "{}", msg);
    }

    /// Mirrors `Assertions.assertNotNull(Object, String)`.
    pub fn assert_not_null<T>(value: Option<&T>, msg: &str) {
        assert!(value.is_some(), "{}", msg);
    }

    /// Mirrors `Assertions.assertNull(Object, String)`.
    pub fn assert_null<T>(value: Option<&T>, msg: &str) {
        assert!(value.is_none(), "{}", msg);
    }

    /// Mirrors `Assertions.errorMessage(String, Object...)`.
    pub fn error_message(msg: &str) -> String {
        msg.to_string()
    }

    /// Mirrors `Assertions.errorMessageContext(String, Object...)`.
    /// Returns the context prefix for error messages.
    pub fn error_message_context(prefix: &str, msg: &str) -> String {
        format!("{}: {}", prefix, msg)
    }

    /// Mirrors `Assertions.assertionEnabled()`.
    /// In Rust, this is analogous to `cfg!(debug_assertions)`.
    pub fn assertion_enabled() -> bool {
        cfg!(debug_assertions)
    }

    /// Mirrors `Assertions.detailAssertionsEnabled()`.
    pub fn detail_assertions_enabled() -> bool {
        cfg!(debug_assertions)
    }
}

/// Mirrors `jdk.graal.compiler.debug.Assertions.AssertionError`.
/// This is a marker for assertion failures.
#[derive(Debug)]
pub struct AssertionError {
    message: String,
}

impl AssertionError {
    pub fn new(message: String) -> Self {
        AssertionError { message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for AssertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AssertionError {}