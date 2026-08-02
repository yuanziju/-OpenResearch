// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2013, 2014, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.code.RegisterSaveLayout`：寄存器到帧槽的映射。
//!
//! 偏离记录：Java `final class RegisterSaveLayout`（持 `Register[]`/`int[]`）→ Rust
//! `pub struct RegisterSaveLayout` 持 `Vec<Register>`/`Vec<i32>`。`TreeMap` 排序映射 →
//! Rust `BTreeMap`。`hashCode` 抛 `UnsupportedOperationException` → Rust 不实现 `Hash`。

use std::collections::BTreeMap;
use std::fmt;

use crate::code::register::Register;

/// 对应 `final class RegisterSaveLayout`。
pub struct RegisterSaveLayout {
    registers: Vec<Register>,
    slots: Vec<i32>,
}

impl RegisterSaveLayout {
    /// 对应 `RegisterSaveLayout(Register[] registers, int[] slots)`。
    pub fn new(registers: Vec<Register>, slots: Vec<i32>) -> Self {
        assert_eq!(registers.len(), slots.len());
        debug_assert_eq!(
            Self::registers_to_slots_inner(&registers, &slots).len(),
            registers.len(),
            "non-unique registers"
        );
        let unique_slots: std::collections::HashSet<i32> = slots.iter().copied().collect();
        debug_assert_eq!(unique_slots.len(), slots.len(), "non-unique slots");
        Self { registers, slots }
    }

    /// 对应 `size()`。
    pub fn size(&self) -> usize {
        self.registers.len()
    }

    /// 对应 `registerToSlot(Register)`。
    pub fn register_to_slot(&self, register: Register) -> i32 {
        for (i, reg) in self.registers.iter().enumerate() {
            if reg == &register {
                return self.slots[i];
            }
        }
        panic!(
            "IllegalArgumentException: {} not saved by this layout: {}",
            register, self
        );
    }

    /// 对应 `registersToSlots(boolean sorted)`。
    pub fn registers_to_slots(&self, sorted: bool) -> BTreeMap<Register, i32> {
        let _ = sorted;
        // Java `sorted=true` 用 `TreeMap`，`sorted=false` 用 `HashMap`。Rust 统一用 `BTreeMap`
        // （`Register: Ord`），迭代顺序确定，对齐 `sorted=true` 语义。
        let mut result = BTreeMap::new();
        for (i, reg) in self.registers.iter().enumerate() {
            result.insert(*reg, self.slots[i]);
        }
        result
    }

    /// 对应 `slotsToRegisters(boolean sorted)`。
    pub fn slots_to_registers(&self, sorted: bool) -> BTreeMap<i32, Register> {
        let _ = sorted;
        let mut result = BTreeMap::new();
        for (i, slot) in self.slots.iter().enumerate() {
            result.insert(*slot, self.registers[i]);
        }
        result
    }

    fn registers_to_slots_inner(registers: &[Register], slots: &[i32]) -> BTreeMap<Register, i32> {
        let mut result = BTreeMap::new();
        for (i, reg) in registers.iter().enumerate() {
            result.insert(*reg, slots[i]);
        }
        result
    }
}

impl fmt::Display for RegisterSaveLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString() { return registersToSlots(true).toString(); }`
        write!(f, "{:?}", self.registers_to_slots(true))
    }
}

impl fmt::Debug for RegisterSaveLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegisterSaveLayout")
            .field("registers", &self.registers)
            .field("slots", &self.slots)
            .finish()
    }
}
