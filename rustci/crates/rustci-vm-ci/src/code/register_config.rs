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
 * version 2 for more details (a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.code.RegisterConfig`：寄存器角色与属性绑定。
//!
//! 偏离记录：Java `interface RegisterConfig` → Rust `pub trait RegisterConfig`。`getCalleeSaveRegisters`
//! 返回 nullable `List<Register>` → `Option<Vec<Register>>`。`getCallingConvention` 的
//! `JavaType[]` → `Vec<Box<dyn JavaType>>`。

use crate::code::architecture::Architecture;
use crate::code::calling_convention::{CallingConvention, Type};
use crate::code::register::Register;
use crate::code::register_attributes::RegisterAttributes;
use crate::code::value_kind_factory::ValueKindFactory;
use crate::meta::java_kind::JavaKind;
use crate::meta::java_type::JavaType;
use crate::meta::platform_kind::PlatformKind;

/// 对应 `interface RegisterConfig`。
pub trait RegisterConfig {
    /// 对应 `getReturnRegister(JavaKind)`。
    fn get_return_register(&self, kind: JavaKind) -> Register;

    /// 对应 `getMaximumFrameSize()`（默认 `Integer.MAX_VALUE`）。
    fn get_maximum_frame_size(&self) -> i32 {
        i32::MAX
    }

    /// 对应 `getFrameRegister()`。
    fn get_frame_register(&self) -> Register;

    /// 对应 `getCallingConvention(Type, JavaType, JavaType[], ValueKindFactory<?>)`。
    /// `value_kind_factory` 为类型擦除的 `dyn ValueKindFactory`（对齐 Java `ValueKindFactory<?>` 通配符）。
    /// `parameter_types` 为 `Vec<&dyn JavaType>`（Java `JavaType[]` 是引用数组；Rust 侧 `JavaType`
    /// 无 `Clone`，`code_util::get_calling_convention` 经 `get_declaring_class`/`get_parameter_type`
    /// 取引用，故参数类型以引用传递，偏离任务规格的 `Vec<Box<dyn JavaType>>`，对齐 Java 引用语义）。
    fn get_calling_convention(
        &self,
        type_: &dyn Type,
        return_type: Option<&dyn JavaType>,
        parameter_types: Vec<&dyn JavaType>,
        value_kind_factory: &dyn ValueKindFactory,
    ) -> CallingConvention;

    /// 对应 `getCallingConventionRegisters(Type, JavaKind)`。
    fn get_calling_convention_registers(&self, type_: &dyn Type, kind: JavaKind) -> Vec<Register>;

    /// 对应 `getAllocatableRegisters()`。
    fn get_allocatable_registers(&self) -> Vec<Register>;

    /// 对应 `filterAllocatableRegisters(PlatformKind, List<Register>)`。
    fn filter_allocatable_registers(
        &self,
        kind: &dyn PlatformKind,
        registers: &[Register],
    ) -> Vec<Register>;

    /// 对应 `getCallerSaveRegisters()`。
    fn get_caller_save_registers(&self) -> Vec<Register>;

    /// 对应 `getCalleeSaveRegisters()`（nullable → `Option`）。
    fn get_callee_save_registers(&self) -> Option<Vec<Register>>;

    /// 对应 `getCalleeSaveRegisterStorageKind(Architecture, Register)`（默认委托
    /// `arch.getLargestStorableKind(register.getRegisterCategory())`）。
    fn get_callee_save_register_storage_kind(
        &self,
        arch: &Architecture,
        callee_save_register: Register,
    ) -> Box<dyn PlatformKind> {
        arch.get_largest_storable_kind(callee_save_register.get_register_category())
    }

    /// 对应 `getAttributesMap()`。
    fn get_attributes_map(&self) -> Vec<RegisterAttributes>;

    /// 对应 `areAllAllocatableRegistersCallerSaved()`。
    fn are_all_allocatable_registers_caller_saved(&self) -> bool;
}
