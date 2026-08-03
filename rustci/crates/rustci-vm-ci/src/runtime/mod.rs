// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2014, 2025, Oracle and/or its affiliates. All rights reserved.
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

//! `rustci_vm_ci::runtime`：镜像 `jdk.vm.ci.runtime`（JVMCI 运行时入口层，5 类）。
//!
//! 命名：模块名 `JVMCI → RustCI`（`rustci_vm_ci::runtime`）；类型名保留 `JVMCI` 前缀
//! 以 1:1 镜像 Java 源（spec §2.2 决议）。方法名/参数名/字段名/方法签名 1:1 保留。

pub mod jvmci;
pub mod jvmci_backend;
pub mod jvmci_compiler;
pub mod jvmci_compiler_factory;
pub mod jvmci_runtime;

#[cfg(test)]
mod mock_tests;
