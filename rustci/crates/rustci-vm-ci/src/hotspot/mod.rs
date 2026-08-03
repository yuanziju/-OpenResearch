// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2025, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.
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

//! `rustci_vm_ci::hotspot`：镜像 `jdk.vm.ci.hotspot` 核心子集（T7）。
//!
//! 移植范围：CompilerToVM native 声明 + 核心接口/class 子集（~20 项）。
//! 本期 skip 79 个文件中的大量辅助类（如 VMFlag、VMSupport、HotSpotVMConfig 等），
//! 只移植核心接口链。

pub mod compiler_to_vm;
pub mod hotspot_code_cache_provider;
pub mod hotspot_compilation_request;
pub mod hotspot_compilation_request_result;
pub mod hotspot_compiled_code;
pub mod hotspot_compiled_nmethod;
pub mod hotspot_constant_pool;
pub mod hotspot_constant_reflection_provider;
pub mod hotspot_installed_code;
pub mod hotspot_jvmci_runtime;
pub mod hotspot_memory_access_provider;
pub mod hotspot_meta_access_provider;
pub mod hotspot_method_wrapper;
pub mod hotspot_nmethod;
pub mod hotspot_resolved_java_method;
pub mod hotspot_resolved_java_method_impl;
pub mod hotspot_resolved_java_type;
pub mod hotspot_resolved_object_type;
pub mod hotspot_resolved_object_type_impl;
pub mod hotspot_speculation_log;
