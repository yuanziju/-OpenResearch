// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2010, 2025, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.code.RegisterAttributes`：寄存器属性集合（caller-save/callee-save/allocatable）。
//!
//! 偏离记录：Java `class RegisterAttributes`（持 `callerSave`/`calleeSave`/`allocatable`）→
//! Rust `pub struct RegisterAttributes`。`createMap` 的 `RegisterAttributes[]` → `Vec<RegisterAttributes>`，
//! `List<Register>` → `&[Register]`/`Vec<Register>`。`NONE` 常量为 `RegisterAttributes::NONE`（
//! `Copy` struct，按值复用）。`registerConfig` 经 `&dyn RegisterConfig` 分派（`dyn` trait 对象）。

use crate::code::register::Register;
use crate::code::register_config::RegisterConfig;

/// 对应 `class RegisterAttributes`。
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RegisterAttributes {
    caller_save: bool,
    callee_save: bool,
    allocatable: bool,
}

impl RegisterAttributes {
    /// 对应 `RegisterAttributes(boolean isCallerSave, boolean isCalleeSave, boolean isAllocatable)`。
    pub fn new(is_caller_save: bool, is_callee_save: bool, is_allocatable: bool) -> Self {
        Self {
            caller_save: is_caller_save,
            callee_save: is_callee_save,
            allocatable: is_allocatable,
        }
    }

    /// 对应 `static final RegisterAttributes NONE = new RegisterAttributes(false, false, false)`。
    pub const NONE: RegisterAttributes = RegisterAttributes {
        caller_save: false,
        callee_save: false,
        allocatable: false,
    };

    /// 对应 `static List<RegisterAttributes> createMap(RegisterConfig, List<Register>)`。
    /// 返回按 `register.number` 索引的属性向量；缺省槽位填 `NONE`。
    pub fn create_map(
        register_config: &dyn RegisterConfig,
        registers: &[Register],
    ) -> Vec<RegisterAttributes> {
        let caller_save_registers = register_config.get_caller_save_registers();
        let callee_save_registers = register_config.get_callee_save_registers();
        let allocatable_registers = register_config.get_allocatable_registers();

        let mut map_len = registers.len();
        for reg in registers {
            if (reg.number as usize) + 1 > map_len {
                map_len = (reg.number as usize) + 1;
            }
        }
        let mut map = vec![Self::NONE; map_len];
        for reg in registers {
            let attr = RegisterAttributes {
                caller_save: caller_save_registers.contains(reg),
                callee_save: callee_save_registers
                    .as_ref()
                    .is_some_and(|cs| cs.contains(reg)),
                allocatable: allocatable_registers.contains(reg),
            };
            map[reg.number as usize] = attr;
        }
        map
    }

    /// 对应 `isAllocatable()`。
    pub fn is_allocatable(&self) -> bool {
        self.allocatable
    }

    /// 对应 `isCalleeSave()`。
    pub fn is_callee_save(&self) -> bool {
        self.callee_save
    }

    /// 对应 `isCallerSave()`。
    pub fn is_caller_save(&self) -> bool {
        self.caller_save
    }
}

impl Default for RegisterAttributes {
    fn default() -> Self {
        Self::NONE
    }
}
