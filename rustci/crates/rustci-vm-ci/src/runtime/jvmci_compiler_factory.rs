// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2015, 2024, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy has been included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have
 * questions.
 */

//! 镜像 `jdk.vm.ci.runtime.JVMCICompilerFactory`：编译器工厂选择。
//!
//! 偏离记录：Java `default void printProperties(PrintStream out)` 的 `java.io.PrintStream`
//! → Rust `&mut dyn std::io::Write`（对齐 Rust IO 抽象）。

use std::io::Write;

use crate::runtime::jvmci_compiler::JVMCICompiler;
use crate::runtime::jvmci_runtime::JVMCIRuntime;

/// 对应 `public interface JVMCICompilerFactory`。
pub trait JVMCICompilerFactory {
    /// 对应 `String getCompilerName()`。
    fn get_compiler_name(&self) -> String;

    /// 对应 `default void onSelection()`。
    fn on_selection(&self) {}

    /// 对应 `JVMCICompiler createCompiler(JVMCIRuntime runtime)`。
    fn create_compiler(&self, runtime: &dyn JVMCIRuntime) -> Box<dyn JVMCICompiler>;

    /// 对应 `default void printProperties(PrintStream out)`。
    fn print_properties(&self, _out: &mut dyn Write) {}
}
