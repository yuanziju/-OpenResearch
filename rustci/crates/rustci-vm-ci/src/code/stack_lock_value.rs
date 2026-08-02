// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2016, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.code.StackLockValue`：调试信息中的锁信息。
//!
//! 偏离记录：Java `final class StackLockValue implements JavaValue`（持可变 `owner`/`slot`
//! 字段）→ Rust `pub struct StackLockValue` 持 `owner: Box<dyn JavaValue>` +
//! `slot: Box<dyn AllocatableValue>`。`setOwner`/`setSlot` 取所有权替换（对齐 Java 可变语义）。
//! `hashCode`（`super.hashCode()` 即 `Object.hashCode`，Java 默认）→ 不实现 `Hash`，
//! `equals` 按 `eliminated && owner.equals && slot.equals`。

use std::any::Any;
use std::fmt;

use crate::meta::allocatable_value::AllocatableValue;
use crate::meta::java_value::JavaValue;
use crate::meta::value::Value;

/// 对应 `final class StackLockValue implements JavaValue`。
pub struct StackLockValue {
    owner: Box<dyn JavaValue>,
    slot: Box<dyn AllocatableValue>,
    eliminated: bool,
}

impl StackLockValue {
    /// 对应 `StackLockValue(JavaValue object, AllocatableValue slot, boolean eliminated)`。
    pub fn new(
        owner: Box<dyn JavaValue>,
        slot: Box<dyn AllocatableValue>,
        eliminated: bool,
    ) -> Self {
        Self {
            owner,
            slot,
            eliminated,
        }
    }

    /// 对应 `getOwner()`。
    pub fn get_owner(&self) -> &dyn JavaValue {
        self.owner.as_ref()
    }

    /// 对应 `setOwner(JavaValue)`。
    pub fn set_owner(&mut self, new_owner: Box<dyn JavaValue>) {
        self.owner = new_owner;
    }

    /// 对应 `getSlot()`。
    pub fn get_slot(&self) -> &dyn AllocatableValue {
        self.slot.as_ref()
    }

    /// 对应 `isEliminated()`。
    pub fn is_eliminated(&self) -> bool {
        self.eliminated
    }

    /// 对应 `setSlot(AllocatableValue)`。
    pub fn set_slot(&mut self, stack_slot: Box<dyn AllocatableValue>) {
        self.slot = stack_slot;
    }

    /// Rust 增设：值相等判定（对齐 Java `equals`）。`owner`/`slot` 为 trait 对象，
    /// 用 `as_any` 下转按具体类型比对；本期仅按引用 identity（对齐 Java `StackLockValue`
    /// 继承 `Object.equals` 的默认语义——Java 源码 `equals` 实际覆写了，但调用 `owner.equals`
    /// 需多态分派，Rust 侧 `JavaValue` 无 `equals` 方法，降级为 identity 比对）。
    pub fn equals(&self, other: &StackLockValue) -> bool {
        self.eliminated == other.eliminated
            && std::ptr::eq(self.owner.as_any(), other.owner.as_any())
            && std::ptr::eq(
                Value::as_any(self.slot.as_ref()) as *const dyn Any,
                Value::as_any(other.slot.as_ref()) as *const dyn Any,
            )
    }
}

impl fmt::Debug for StackLockValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StackLockValue")
            .field("owner", &self.owner)
            .field("slot", &self.slot)
            .field("eliminated", &self.eliminated)
            .finish()
    }
}

impl fmt::Display for StackLockValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString()`：`monitor[owner, slot, eliminated]`。`owner` 为 `dyn JavaValue`
        // （`JavaValue` 无 `Display` 超 trait），用 `{:?}` 字符串化（偏离 Java `toString`，
        // 与 `ReferenceMap`/`tabulateValues` 等路径约定一致）。
        write!(f, "monitor[{:?}", self.owner)?;
        write!(f, ", {}", self.slot)?;
        if self.eliminated {
            f.write_str(", eliminated")?;
        }
        f.write_str("]")
    }
}

impl JavaValue for StackLockValue {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
