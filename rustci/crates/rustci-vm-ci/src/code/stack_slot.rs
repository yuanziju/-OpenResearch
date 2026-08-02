// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2010, 2016, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details.
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.code.StackSlot`：编译器溢出槽或基于栈的出入参位置。
//!
//! 偏离记录：Java `final class StackSlot extends AllocatableValue`
//! → Rust `pub struct StackSlot` 持 `kind: Box<dyn ValueKind>` + `offset: i32` +
//! `add_frame_size: bool`，实现 `Value` + `AllocatableValue` + `JavaValue`（组合替代继承）。
//! 私有构造器 + 静态工厂 `get` 对齐 Java。`hashCode`/`equals` 覆写含 super 逻辑。

use std::any::Any;
use std::fmt;

use crate::meta::allocatable_value::AllocatableValue;
use crate::meta::java_value::JavaValue;
use crate::meta::value::Value;
use crate::meta::value_kind::ValueKind;

/// 对应 `final class StackSlot extends AllocatableValue`。
pub struct StackSlot {
    kind: Box<dyn ValueKind>,
    offset: i32,
    add_frame_size: bool,
}

impl StackSlot {
    /// 对应 `static StackSlot get(ValueKind<?>, int offset, boolean addFrameSize)`。
    pub fn get(kind: Box<dyn ValueKind>, offset: i32, add_frame_size: bool) -> Self {
        debug_assert!(add_frame_size || offset >= 0);
        Self {
            kind,
            offset,
            add_frame_size,
        }
    }

    /// 对应 `getOffset(int totalFrameSize)`。
    pub fn get_offset(&self, total_frame_size: i32) -> i32 {
        debug_assert!(total_frame_size > 0 || !self.add_frame_size);
        let result = self.offset
            + if self.add_frame_size {
                total_frame_size
            } else {
                0
            };
        debug_assert!(result >= 0);
        result
    }

    /// 对应 `isInCallerFrame()`。
    pub fn is_in_caller_frame(&self) -> bool {
        self.add_frame_size && self.offset >= 0
    }

    /// 对应 `getRawOffset()`。
    pub fn get_raw_offset(&self) -> i32 {
        self.offset
    }

    /// 对应 `getRawAddFrameSize()`。
    pub fn get_raw_add_frame_size(&self) -> bool {
        self.add_frame_size
    }

    /// 对应 `asOutArg()`：返回 `addFrameSize=false` 的等价槽（Java 侧 `addFrameSize` 已为
    /// false 时返回 `this`，Rust 侧始终构造等价新实例，行为一致）。
    pub fn as_out_arg(&self) -> Self {
        debug_assert!(self.offset >= 0);
        Self::get(self.kind.clone_box(), self.offset, false)
    }

    /// 对应 `asInArg()`：返回 `addFrameSize=true` 的等价槽。
    pub fn as_in_arg(&self) -> Self {
        debug_assert!(self.offset >= 0);
        Self::get(self.kind.clone_box(), self.offset, true)
    }
}

impl fmt::Debug for StackSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StackSlot")
            .field("kind", &self.kind)
            .field("offset", &self.offset)
            .field("add_frame_size", &self.add_frame_size)
            .finish()
    }
}

impl fmt::Display for StackSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.add_frame_size {
            write!(f, "out:{}{}", self.offset, self.get_kind_suffix())
        } else if self.offset >= 0 {
            write!(f, "in:{}{}", self.offset, self.get_kind_suffix())
        } else {
            write!(f, "stack:{}{}", -self.offset, self.get_kind_suffix())
        }
    }
}

impl JavaValue for StackSlot {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Value for StackSlot {
    fn get_value_kind(&self) -> &dyn ValueKind {
        self.kind.as_ref()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn value_equals(&self, other: &dyn Value) -> bool {
        match other.as_any().downcast_ref::<StackSlot>() {
            None => false,
            Some(o) => {
                // super.equals（按 valueKind.equals）+ addFrameSize + offset。
                self.kind.kind_equals(o.kind.as_ref())
                    && self.add_frame_size == o.add_frame_size
                    && self.offset == o.offset
            }
        }
    }

    fn value_hash(&self) -> u64 {
        // super.hashCode() = 41 + valueKind.hashCode()。
        let super_h = 41u64.wrapping_add(self.kind.kind_hash());
        let mut result = 37u64.wrapping_mul(super_h);
        result = result.wrapping_add(if self.add_frame_size { 1231 } else { 1237 });
        result = 37u64.wrapping_mul(result).wrapping_add(self.offset as u64);
        result
    }
}

impl AllocatableValue for StackSlot {}
