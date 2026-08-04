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
// Rust mirror of `jdk.graal.compiler.debug.CSVUtil`.

/// Mirrors `jdk.graal.compiler.debug.CSVUtil`.
/// Utilities for formatting CSV (Comma-Separated Values) output.
pub struct CsvUtil;

impl CsvUtil {
    /// The separator character used in CSV output. Mirrors `CSVUtil.SEPARATOR`.
    pub const SEPARATOR: char = ';';

    /// The quote character used for escaping. Mirrors `CSVUtil.QUOTE`.
    pub const QUOTE: char = '"';

    /// The escape character. Mirrors `CSVUtil.ESCAPE`.
    pub const ESCAPE: char = '\\';

    /// Escapes a string for CSV output by wrapping it in quotes if it contains
    /// special characters. Mirrors `CSVUtil.escape(String)`.
    pub fn escape(value: &str) -> String {
        if value.is_empty() {
            return format!("{}{}", Self::QUOTE, Self::QUOTE);
        }
        if value.contains(Self::SEPARATOR) || value.contains(Self::QUOTE) || value.contains('\n') {
            let escaped = value
                .replace(Self::ESCAPE, &format!("{}{}", Self::ESCAPE, Self::ESCAPE))
                .replace(Self::QUOTE, &format!("{}{}", Self::ESCAPE, Self::QUOTE));
            format!("{}{}{}", Self::QUOTE, escaped, Self::QUOTE)
        } else {
            value.to_string()
        }
    }

    /// Formats a row of CSV values. Mirrors `CSVUtil.format(String, Object...)` pattern.
    pub fn format(values: &[&str]) -> String {
        let escaped: Vec<String> = values.iter().map(|v| Self::escape(v)).collect();
        escaped.join(&Self::SEPARATOR.to_string())
    }

    /// Builds a CSV line from a format string and arguments, mirroring
    /// `CSVUtil.buildFormatString(String, Object...)`.
    pub fn build_format_string(format_str: &str, args: &[&str]) -> String {
        let mut result = format_str.to_string();
        for arg in args {
            if let Some(pos) = result.find("{}") {
                let escaped = Self::escape(arg);
                result.replace_range(pos..pos + 2, &escaped);
            }
        }
        result
    }

    /// Converts a duration in nanoseconds to a CSV-compatible string.
    /// Mirrors `CSVUtil.toSeconds(long)`.
    pub fn to_seconds(nanos: u64) -> String {
        format!("{:.6}", nanos as f64 / 1_000_000_000.0)
    }

    /// Converts a byte count to a human-readable CSV value.
    /// Mirrors `CSVUtil.toKB(long)`.
    pub fn to_kb(bytes: u64) -> String {
        format!("{:.3}", bytes as f64 / 1024.0)
    }

    /// Converts a byte count to MB. Mirrors `CSVUtil.toMB(long)`.
    pub fn to_mb(bytes: u64) -> String {
        format!("{:.3}", bytes as f64 / (1024.0 * 1024.0))
    }
}