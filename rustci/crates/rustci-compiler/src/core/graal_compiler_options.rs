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
// Rust mirror of `jdk.graal.compiler.core.GraalCompilerOptions`.
// All static final fields → struct + associated constants.

use crate::core::compilation_wrapper::ExceptionAction;

/// Options related to `GraalCompiler`.
/// Mirrors the static final fields of `GraalCompilerOptions`.
pub struct GraalCompilerOptions;

impl GraalCompilerOptions {
    /// Print an informational line to the console for each completed compilation.
    pub const PRINT_COMPILATION_DEFAULT: bool = false;

    /// Print statistics for each completed compilation to a CSV file.
    pub const PRINT_COMPILATION_CSV_DEFAULT: Option<&'static str> = None;

    /// Pattern for method(s) that will trigger an exception when compiled.
    pub const CRASH_AT_DEFAULT: Option<&'static str> = None;

    /// Treats compilation bailouts as compilation failures.
    pub const COMPILATION_BAILOUT_AS_FAILURE_DEFAULT: bool = false;

    /// Specifies the action to take when compilation fails.
    pub const COMPILATION_FAILURE_ACTION_DEFAULT: ExceptionAction = ExceptionAction::Silent;

    /// Maximum number of compilation failures to handle before changing to a less verbose action.
    pub const MAX_COMPILATION_PROBLEMS_PER_ACTION_DEFAULT: i32 = 2;

    /// Systemic compilation failure rate threshold percentage.
    pub const SYSTEMIC_COMPILATION_FAILURE_RATE_DEFAULT: i32 = 1;

    /// Number of seconds by which to slow down each compilation.
    pub const INJECTED_COMPILATION_DELAY_DEFAULT: i32 = 0;

    /// Phase filter key for heap dump after phases.
    pub const DUMP_HEAP_AFTER_DEFAULT: &'static str = "<compilation>";
}
