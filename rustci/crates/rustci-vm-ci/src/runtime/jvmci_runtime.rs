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

//! 镜像 `jdk.vm.ci.runtime.JVMCIRuntime`：宿主访问入口。
//!
//! 偏离记录：
//! - Java `<T extends Architecture> JVMCIBackend getJVMCIBackend(Class<T> arch)` 的泛型 +
//!   `Class<T>` 参数无 Rust 对应：Rust 用 `&dyn Architecture` 参数（运行时按架构实例判断）。
//! - Java 返回 `JVMCIBackend`（找到时）/ `null`（找不到）；Rust 返回
//!   `Option<&JVMCIBackend>`（`None` 表达 null）。
//! - `getHostJVMCIBackend`/`getJVMCIBackend` 返回 `&JVMCIBackend`（引用，对齐 Java 返回
//!   单例引用语义），而非 `Box<dyn JVMCIBackend>`：JVMCIBackend 为 struct（非 trait），
//!   `dyn JVMCIBackend` 不合法；且 JVMCIBackend 持非 `Clone` 的 `Box<dyn ...>` 字段，
//!   无法每次构造 owned `Box`。返回引用对齐 Java `getHostJVMCIBackend` 返回存储后端引用语义。

use crate::code::architecture::Architecture;
use crate::runtime::jvmci_backend::JVMCIBackend;
use crate::runtime::jvmci_compiler::JVMCICompiler;

/// 对应 `public interface JVMCIRuntime`。
pub trait JVMCIRuntime {
    /// 对应 `JVMCICompiler getCompiler()`。
    fn get_compiler(&self) -> Box<dyn JVMCICompiler>;

    /// 对应 `JVMCIBackend getHostJVMCIBackend()`。
    fn get_host_jvmci_backend(&self) -> &JVMCIBackend;

    /// 对应 `<T extends Architecture> JVMCIBackend getJVMCIBackend(Class<T> arch)`。
    fn get_jvmci_backend(&self, arch: &Architecture) -> Option<&JVMCIBackend>;
}
