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
// Rust mirror of `jdk.graal.compiler.debug.DebugConfig`.

use std::any::Any;

use crate::debug::debug_dump_handler::DebugDumpHandler;
use crate::debug::debug_verify_handler::DebugVerifyHandler;
use crate::debug::java_method_context::JavaMethodContext;

/// Mirrors `jdk.graal.compiler.debug.DebugConfig`.
/// Configuration interface for the debug framework.
pub trait DebugConfig {
    /// Returns the debug filter for method-level filtering.
    /// Mirrors `DebugConfig.getDebugFilter()`.
    fn get_debug_filter(&self) -> Option<&dyn DebugFilterTrait>;

    /// Returns the log stream for debug output.
    /// Mirrors `DebugConfig.logStream()`.
    fn log_stream(&self) -> Option<&dyn std::io::Write>;

    /// Returns the dump handlers for graph dumping.
    /// Mirrors `DebugConfig.dumpHandlers()`.
    fn dump_handlers(&self) -> &[Box<dyn DebugDumpHandler>];

    /// Returns the verify handlers for graph verification.
    /// Mirrors `DebugConfig.verifyHandlers()`.
    fn verify_handlers(&self) -> &[Box<dyn DebugVerifyHandler>];

    /// Returns the directory for diagnostic output.
    /// Mirrors `DebugConfig.getDiagnosticsOutputDirectory()`.
    fn get_diagnostics_output_directory(&self) -> Option<String>;

    /// Returns whether dumping is enabled.
    /// Mirrors `DebugConfig.isDumpEnabled()`.
    fn is_dump_enabled(&self) -> bool;

    /// Returns whether logging is enabled.
    /// Mirrors `DebugConfig.isLogEnabled()`.
    fn is_log_enabled(&self) -> bool;

    /// Returns whether counting is enabled.
    /// Mirrors `DebugConfig.isCountEnabled()`.
    fn is_count_enabled(&self) -> bool;

    /// Returns whether timing is enabled.
    /// Mirrors `DebugConfig.isTimeEnabled()`.
    fn is_time_enabled(&self) -> bool;

    /// Returns whether memory tracking is enabled.
    /// Mirrors `DebugConfig.isMemUseTrackingEnabled()`.
    fn is_mem_use_tracking_enabled(&self) -> bool;

    /// Returns whether verification is enabled.
    /// Mirrors `DebugConfig.isVerifyEnabled()`.
    fn is_verify_enabled(&self) -> bool;

    /// Returns whether method metrics are enabled.
    /// Mirrors `DebugConfig.isMethodMeterEnabled()`.
    fn is_method_meter_enabled(&self) -> bool;

    /// Returns an object representing the compilation context.
    /// Mirrors `DebugConfig.getCompilationContext()`.
    fn get_compilation_context(&self) -> Option<Box<dyn Any>>;
}

/// Mirrors the `DebugFilter` interface used by `DebugConfig`.
/// Extracted as a separate trait for modularity.
pub trait DebugFilterTrait {
    /// Returns whether the given method matches the filter.
    /// Mirrors `DebugFilter.matchMethod(JavaMethodContext)`.
    fn match_method(&self, method: &dyn JavaMethodContext) -> bool;
}