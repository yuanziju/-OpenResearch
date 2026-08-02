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

//! `rustci_vm_ci::code`：镜像 `jdk.vm.ci.code` 核心接口与 final class 子集。

pub mod architecture;
pub mod bailout_exception;
pub mod bytecode_frame;
pub mod bytecode_position;
pub mod calling_convention;
pub mod code_cache_provider;
pub mod code_util;
pub mod compilation_request;
pub mod compilation_request_result;
pub mod compiled_code;
pub mod cpu_feature_name;
pub mod debug_info;
pub mod installed_code;
pub mod invalid_installed_code_exception;
pub mod location;
pub mod memory_barriers;
pub mod reference_map;
pub mod register;
pub mod register_attributes;
pub mod register_config;
pub mod register_save_layout;
pub mod register_value;
pub mod site;
pub mod stack;
pub mod stack_lock_value;
pub mod stack_slot;
pub mod target_description;
pub mod value_kind_factory;
pub mod value_util;
pub mod virtual_object;

#[cfg(test)]
mod mock_tests;
