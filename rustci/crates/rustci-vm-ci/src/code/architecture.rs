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
 * version 2 for more details (a copy has been included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Packaging, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.code.Architecture`（`abstract class`）：CPU 架构描述。
//!
//! 偏离记录：
//! - Java `abstract class Architecture`（带 `wordKind`/`name`/`registers`/`byteOrder` 等字段 +
//!   4 个 abstract 方法）→ Rust `pub struct Architecture` 持全部字段 + `sub: Box<dyn ArchitectureSub>`。
//!   Rust 无继承，abstract 方法经 `ArchitectureSub` trait 委托（组合 + 委托替代继承）。构造器
//!   `protected` → `pub(crate)`，校验 `registers[i].number == i` 否则 panic（对齐 Java `JVMCIError`）。
//! - Java `java.nio.ByteOrder` → 同文件 `pub enum ByteOrder`（`BigEndian`/`LittleEndian`）。
//! - `Set<? extends CPUFeatureName>` → `Vec<Box<dyn CPUFeatureName>>`。
//! - `equals`/`hashCode`（Java `final`，按 `name`）→ `equals(&self, &Architecture)`；不实现 `Hash`
//!   （字段含 `Box<dyn ...>`，非本期需要）。
//! - `getPlatformKind` 返回 nullable `PlatformKind` → `Option<Box<dyn PlatformKind>>`。

use std::fmt;

use crate::code::cpu_feature_name::CPUFeatureName;
use crate::code::register::{Register, RegisterCategory};
use crate::meta::java_kind::JavaKind;
use crate::meta::platform_kind::PlatformKind;

/// 对应 `java.nio.ByteOrder`。
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

/// 对应 `Architecture` 的 abstract 方法集合。子类实现本 trait 并经 `Architecture::new`
/// 注入（组合 + 委托替代 Java 继承）。
pub trait ArchitectureSub {
    /// 对应 `abstract Set<? extends CPUFeatureName> getFeatures()`。
    fn get_features(&self) -> Vec<Box<dyn CPUFeatureName>>;

    /// 对应 `abstract boolean canStoreValue(RegisterCategory, PlatformKind)`。
    fn can_store_value(&self, category: RegisterCategory, kind: &dyn PlatformKind) -> bool;

    /// 对应 `abstract PlatformKind getLargestStorableKind(RegisterCategory)`。
    fn get_largest_storable_kind(&self, category: RegisterCategory) -> Box<dyn PlatformKind>;

    /// 对应 `abstract PlatformKind getPlatformKind(JavaKind)`（nullable → `Option`）。
    fn get_platform_kind(&self, java_kind: JavaKind) -> Option<Box<dyn PlatformKind>>;
}

/// 对应 `abstract class Architecture`。
pub struct Architecture {
    word_kind: Box<dyn PlatformKind>,
    name: String,
    registers: Vec<Register>,
    byte_order: ByteOrder,
    unaligned_memory_access: bool,
    implicit_memory_barriers: i32,
    machine_code_call_displacement_offset: i32,
    return_address_size: i32,
    sub: Box<dyn ArchitectureSub>,
}

impl Architecture {
    /// 对应 `protected Architecture(String name, PlatformKind wordKind, ByteOrder byteOrder,
    /// boolean unalignedMemoryAccess, List<Register> registers, int implicitMemoryBarriers,
    /// int nativeCallDisplacementOffset, int returnAddressSize)`。
    ///
    /// 偏离：Java `protected` → Rust `pub`（Rust 无 `protected` 等价物；字段为私有，构造器公开
    /// 供其他 crate 的子类经 `ArchitectureSub` trait 注入使用）。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        word_kind: Box<dyn PlatformKind>,
        byte_order: ByteOrder,
        unaligned_memory_access: bool,
        registers: Vec<Register>,
        implicit_memory_barriers: i32,
        native_call_displacement_offset: i32,
        return_address_size: i32,
        sub: Box<dyn ArchitectureSub>,
    ) -> Self {
        // 对应构造器校验：registers[i].number == i，否则抛 JVMCIError。
        for (i, reg) in registers.iter().enumerate() {
            if reg.number as usize != i {
                panic!("JVMCIError: {}: {} != {}", reg, reg.number, i);
            }
        }
        Self {
            word_kind,
            name,
            registers,
            byte_order,
            unaligned_memory_access,
            implicit_memory_barriers,
            machine_code_call_displacement_offset: native_call_displacement_offset,
            return_address_size,
            sub,
        }
    }

    /// 对应 `abstract Set<? extends CPUFeatureName> getFeatures()`。
    pub fn get_features(&self) -> Vec<Box<dyn CPUFeatureName>> {
        self.sub.get_features()
    }

    /// 对应 `getWordSize()`。
    pub fn get_word_size(&self) -> i32 {
        self.word_kind.get_size_in_bytes()
    }

    /// 对应 `getWordKind()`。
    pub fn get_word_kind(&self) -> &dyn PlatformKind {
        self.word_kind.as_ref()
    }

    /// 对应 `getName()`。
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// 对应 `getRegisters()`。
    pub fn get_registers(&self) -> &[Register] {
        &self.registers
    }

    /// 对应 `getAvailableValueRegisters()`（默认返回 `getRegisters()`）。
    pub fn get_available_value_registers(&self) -> Vec<Register> {
        self.registers.clone()
    }

    /// 对应 `getByteOrder()`。
    pub fn get_byte_order(&self) -> ByteOrder {
        self.byte_order
    }

    /// 对应 `supportsUnalignedMemoryAccess()`。
    pub fn supports_unaligned_memory_access(&self) -> bool {
        self.unaligned_memory_access
    }

    /// 对应 `getReturnAddressSize()`。
    pub fn get_return_address_size(&self) -> i32 {
        self.return_address_size
    }

    /// 对应 `getMachineCodeCallDisplacementOffset()`。
    pub fn get_machine_code_call_displacement_offset(&self) -> i32 {
        self.machine_code_call_displacement_offset
    }

    /// 对应 `final int requiredBarriers(int barriers)`。
    pub fn required_barriers(&self, barriers: i32) -> i32 {
        barriers & !self.implicit_memory_barriers
    }

    /// 对应 `abstract boolean canStoreValue(RegisterCategory, PlatformKind)`。
    pub fn can_store_value(&self, category: RegisterCategory, kind: &dyn PlatformKind) -> bool {
        self.sub.can_store_value(category, kind)
    }

    /// 对应 `abstract PlatformKind getLargestStorableKind(RegisterCategory)`。
    pub fn get_largest_storable_kind(&self, category: RegisterCategory) -> Box<dyn PlatformKind> {
        self.sub.get_largest_storable_kind(category)
    }

    /// 对应 `abstract PlatformKind getPlatformKind(JavaKind)`（nullable → `Option`）。
    pub fn get_platform_kind(&self, java_kind: JavaKind) -> Option<Box<dyn PlatformKind>> {
        self.sub.get_platform_kind(java_kind)
    }

    /// 对应 `final boolean equals(Object)`（按 `name`）。
    pub fn equals(&self, other: &Architecture) -> bool {
        if std::ptr::eq(self, other) {
            return true;
        }
        if self.name == other.name {
            debug_assert_eq!(self.byte_order, other.byte_order);
            debug_assert_eq!(
                self.implicit_memory_barriers,
                other.implicit_memory_barriers
            );
            debug_assert_eq!(
                self.machine_code_call_displacement_offset,
                other.machine_code_call_displacement_offset
            );
            debug_assert_eq!(self.registers, other.registers);
            debug_assert_eq!(self.return_address_size, other.return_address_size);
            debug_assert_eq!(self.unaligned_memory_access, other.unaligned_memory_access);
            true
        } else {
            false
        }
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `final String toString()`：`getName().toLowerCase()`。
        f.write_str(&self.name.to_lowercase())
    }
}

impl fmt::Debug for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Architecture")
            .field("name", &self.name)
            .field("byte_order", &self.byte_order)
            .field("registers", &self.registers.len())
            .finish_non_exhaustive()
    }
}
