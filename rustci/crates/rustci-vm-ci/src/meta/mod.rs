// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2024, Oracle and/or its affiliates. All rights reserved.
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

//! `rustci_vm_ci::meta`：镜像 `jdk.vm.ci.meta` 核心接口与 final class 子集（T4）。

pub mod abstract_java_profile;
pub mod annotated;
pub mod annotation_data;
pub mod assumptions;
pub mod byte_buffer;
pub mod constant;
pub mod constant_pool;
pub mod constant_reflection;
pub mod default_profiling_info;
pub mod deoptimization;
pub mod enum_data;
pub mod error_data;
pub mod exception_handler;
pub mod invoke_target;
pub mod java_constant;
pub mod java_field;
pub mod java_kind;
pub mod java_method;
pub mod java_method_profile;
pub mod java_reflect;
pub mod java_type;
pub mod java_type_profile;
pub mod java_value;
pub mod line_number_table;
pub mod local;
pub mod local_variable_table;
pub mod memory_access;
pub mod meta_access;
pub mod meta_util;
pub mod method_handle_access;
pub mod modifiers_provider;
pub mod null_constant;
pub mod platform_kind;
pub mod primitive_constant;
pub mod profiling_info;
pub mod raw_constant;
pub mod resolved_java_field;
pub mod resolved_java_method;
pub mod resolved_java_type;
pub mod serializable_constant;
pub mod signature;
pub mod speculation_log;
pub mod tri_state;
pub mod unresolved_java_field;
pub mod unresolved_java_method;
pub mod unresolved_java_type;
pub mod vm_constant;

#[cfg(test)]
mod mock_tests;
