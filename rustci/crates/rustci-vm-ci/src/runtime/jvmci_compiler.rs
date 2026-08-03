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

//! 镜像 `jdk.vm.ci.runtime.JVMCICompiler`：JVMCI 编译器接口。
//!
//! 偏离记录：Java 接口常量 `int INVOCATION_ENTRY_BCI = -1` → Rust 模块级 `pub const`。
//! 原因：Rust trait 关联 `const` 使 trait 非 dyn 兼容（`dyn JVMCICompiler` 不合法），
//! 而 `JVMCIRuntime::get_compiler` 返回 `Box<dyn JVMCICompiler>` 需要 dyn 兼容。
//! 模块级 const 的访问路径 `jvmci_compiler::INVOCATION_ENTRY_BCI` 对齐 Java
//! `JVMCICompiler.INVOCATION_ENTRY_BCI` 的类型名访问语义。

use crate::code::compilation_request::CompilationRequest;
use crate::code::compilation_request_result::CompilationRequestResult;

/// 对应 `int INVOCATION_ENTRY_BCI = -1`（接口常量）。
pub const INVOCATION_ENTRY_BCI: i32 = -1;

/// 对应 `public interface JVMCICompiler`。
pub trait JVMCICompiler {
    /// 对应 `CompilationRequestResult compileMethod(CompilationRequest request)`。
    fn compile_method(&self, request: &CompilationRequest) -> Box<dyn CompilationRequestResult>;

    /// 对应 `default boolean isGCSupported(int gcIdentifier)`：默认 `true`。
    fn is_gc_supported(&self, _gc_identifier: i32) -> bool {
        true
    }

    /// 对应 `default boolean isIntrinsicSupported(int intrinsicIdentifier)`：默认 `false`。
    fn is_intrinsic_supported(&self, _intrinsic_identifier: i32) -> bool {
        false
    }
}
