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
// Rust mirror of `jdk.graal.compiler.debug.DebugFilter`.

use crate::debug::debug_config::DebugFilterTrait;
use crate::debug::java_method_context::JavaMethodContext;

/// Mirrors `jdk.graal.compiler.debug.DebugFilter`.
/// Filters debug output based on method-level patterns.
pub struct DebugFilter {
    /// The filter pattern string. Mirrors `DebugFilter.filter`.
    pattern: String,
    /// Whether the filter is inverted (exclude rather than include).
    /// Mirrors the negation logic in Java.
    exclude: bool,
}

impl DebugFilter {
    /// Creates a new DebugFilter with the given pattern.
    /// Mirrors `DebugFilter(String)`.
    pub fn new(pattern: String) -> Self {
        let exclude = pattern.starts_with('~');
        let pattern = if exclude {
            pattern[1..].to_string()
        } else {
            pattern
        };
        DebugFilter { pattern, exclude }
    }

    /// Returns the filter pattern. Mirrors `DebugFilter.getFilter()`.
    pub fn get_filter(&self) -> &str {
        &self.pattern
    }

    /// Returns whether the filter is an exclusion filter.
    pub fn is_exclude(&self) -> bool {
        self.exclude
    }

    /// Matches a string against the filter pattern.
    /// Mirrors `DebugFilter.matchPattern(String)`.
    fn matches_pattern(&self, input: &str) -> bool {
        if self.pattern.is_empty() || self.pattern == "*" {
            return true;
        }
        if self.pattern.contains('*') {
            let parts: Vec<&str> = self.pattern.split('*').collect();
            let mut remaining = input;
            for (i, part) in parts.iter().enumerate() {
                if part.is_empty() {
                    if i == parts.len() - 1 {
                        return true;
                    }
                    continue;
                }
                if let Some(pos) = remaining.find(part) {
                    remaining = &remaining[pos + part.len()..];
                } else {
                    return false;
                }
            }
            true
        } else {
            input.contains(&self.pattern)
        }
    }
}

impl DebugFilterTrait for DebugFilter {
    fn match_method(&self, method: &dyn JavaMethodContext) -> bool {
        let method_name = match method.as_java_method() {
            Some(name) => name,
            None => return false,
        };
        let matched = self.matches_pattern(method_name);
        if self.exclude {
            !matched
        } else {
            matched
        }
    }
}

impl Default for DebugFilter {
    fn default() -> Self {
        DebugFilter::new("*".to_string())
    }
}