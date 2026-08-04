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
// Rust mirror of `jdk.graal.compiler.debug.Indent`.

use std::cell::Cell;

/// Mirrors `jdk.graal.compiler.debug.Indent`.
/// Manages indentation level for formatted debug output.
pub struct Indent {
    /// The current indentation level. Mirrors `Indent.indent`.
    level: Cell<usize>,
    /// The string used for each indentation level. Mirrors `Indent.INDENTATION`.
    indent_str: String,
}

impl Indent {
    /// The default indentation string. Mirrors `Indent.INDENTATION`.
    pub const DEFAULT_INDENTATION: &str = "  ";

    /// Creates a new Indent with default indentation. Mirrors `Indent()`.
    pub fn new() -> Self {
        Indent {
            level: Cell::new(0),
            indent_str: Self::DEFAULT_INDENTATION.to_string(),
        }
    }

    /// Creates a new Indent with a custom indentation string.
    pub fn with_indent_str(indent_str: String) -> Self {
        Indent {
            level: Cell::new(0),
            indent_str,
        }
    }

    /// Returns the current indentation as a string. Mirrors `Indent.toString()`.
    pub fn to_string(&self) -> String {
        self.indent_str.repeat(self.level.get())
    }

    /// Increments the indentation level. Mirrors `Indent.inline()`.
    /// Returns the old level for use in try-with-resources patterns.
    pub fn indent(&self) -> usize {
        let old = self.level.get();
        self.level.set(old + 1);
        old
    }

    /// Decrements the indentation level. Mirrors `Indent.outdent()`.
    pub fn outdent(&self) {
        let level = self.level.get();
        if level > 0 {
            self.level.set(level - 1);
        }
    }

    /// Returns the current indentation level. Mirrors `Indent.getLevel()`.
    pub fn get_level(&self) -> usize {
        self.level.get()
    }

    /// Resets the indentation to zero. Mirrors setting indent to 0.
    pub fn reset(&self) {
        self.level.set(0);
    }

    /// Creates a guard that increments indentation on creation and decrements
    /// on drop. Mirrors the try-with-resources pattern in Java.
    pub fn scoped_indent(&self) -> IndentGuard {
        let old = self.indent();
        IndentGuard {
            indent: self,
            old_level: old,
        }
    }
}

impl Default for Indent {
    fn default() -> Self {
        Indent::new()
    }
}

/// Guard that restores the indentation level when dropped.
pub struct IndentGuard<'a> {
    indent: &'a Indent,
    #[allow(dead_code)]
    old_level: usize,
}

impl<'a> Drop for IndentGuard<'a> {
    fn drop(&mut self) {
        self.indent.outdent();
    }
}

/// Formats a string with indentation. Mirrors `Indent.format(String)`.
pub fn indent_format(indent: &Indent, s: &str) -> String {
    let prefix = indent.to_string();
    let mut result = String::with_capacity(s.len() + prefix.len() * s.lines().count());
    for line in s.lines() {
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(&prefix);
        result.push_str(line);
    }
    result
}