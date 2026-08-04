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
// Rust mirror of `jdk.graal.compiler.debug.DebugOptions`.

/// Mirrors `jdk.graal.compiler.debug.DebugOptions`.
/// Debug-related option keys for configuring the debug framework.
pub struct DebugOptions;

impl DebugOptions {
    /// Option for enabling/disabling verbose debug output. Mirrors `DebugOptions.Log`.
    pub const LOG: &str = "Log";

    /// Option for the debug filter pattern. Mirrors `DebugOptions.Filter`.
    pub const FILTER: &str = "Filter";

    /// Option for enabling method-level metrics. Mirrors `DebugOptions.MethodMeter`.
    pub const METHOD_METER: &str = "MethodMeter";

    /// Option for enabling timers. Mirrors `DebugOptions.Time`.
    pub const TIME: &str = "Time";

    /// Option for enabling counters. Mirrors `DebugOptions.Count`.
    pub const COUNT: &str = "Count";

    /// Option for enabling memory use tracking. Mirrors `DebugOptions.TrackMemUse`.
    pub const TRACK_MEM_USE: &str = "TrackMemUse";

    /// Option for enabling graph dumping. Mirrors `DebugOptions.Dump`.
    pub const DUMP: &str = "Dump";

    /// Option for enabling graph verification. Mirrors `DebugOptions.Verify`.
    pub const VERIFY: &str = "Verify";

    /// Option for the dump path. Mirrors `DebugOptions.DumpPath`.
    pub const DUMP_PATH: &str = "DumpPath";

    /// Option for printing the graph on crash. Mirrors `DebugOptions.PrintGraph`.
    pub const PRINT_GRAPH: &str = "PrintGraph";

    /// Option for the maximum number of graph dumps. Mirrors `DebugOptions.MaxDumps`.
    pub const MAX_DUMPS: &str = "MaxDumps";

    /// Option for enabling CSV output. Mirrors `DebugOptions.CSV`.
    pub const CSV: &str = "CSV";

    /// Option for the CSV output directory. Mirrors `DebugOptions.CsvDir`.
    pub const CSV_DIR: &str = "CsvDir";

    /// Returns the option value as a string, with a default fallback.
    /// Mirrors the pattern of looking up option values in `OptionValues`.
    pub fn get_string_value(option_values: &dyn DebugOptionValues, name: &str, default: &str) -> String {
        option_values.get(name).unwrap_or(default).to_string()
    }

    /// Returns the option value as a boolean.
    pub fn get_bool_value(option_values: &dyn DebugOptionValues, name: &str, default: bool) -> bool {
        option_values
            .get(name)
            .map(|v| v == "true" || v == "True")
            .unwrap_or(default)
    }

    /// Returns the option value as an integer.
    pub fn get_int_value(option_values: &dyn DebugOptionValues, name: &str, default: i64) -> i64 {
        option_values
            .get(name)
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(default)
    }
}

/// Trait for accessing debug option values. Mirrors the `OptionValues` interface.
pub trait DebugOptionValues {
    /// Returns the value for the given option name, or None if not set.
    fn get(&self, name: &str) -> Option<&str>;
}