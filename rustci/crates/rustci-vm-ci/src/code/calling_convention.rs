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

//! 镜像 `jdk.vm.ci.code.CallingConvention`：调用约定的描述（参数/返回值位置）。
//!
//! 偏离记录：Java `class CallingConvention`（持 `stackSize`/`returnLocation`/`argumentLocations`）
//! → Rust `pub struct CallingConvention`。`AllocatableValue...` 变长参数 → `Vec<Box<dyn AllocatableValue>>`。
//! 私有 `verify()` 的 `assert isStackSlot || isAllocatableValue` 由 Rust 类型系统保证（Vec 元素
//! 均为 `AllocatableValue`），省略运行时断言。嵌套 `interface Type`（标记）→ 同文件 `pub trait Type`。
//! `toString` 中 `Value.ILLEGAL.equals(returnLocation)` 判别：`IllegalValue.equals` 按 `instanceof`
//! 判等，等价于 `returnLocation` 的 `valueKind` 为 `IllegalValueKind`，故经 `get_value_kind()`
//! （仅 `Value` trait 定义，无歧义）取 kind 后下转 `IllegalValueKind` 判别。

use std::fmt;

use crate::meta::allocatable_value::AllocatableValue;
use crate::meta::value_kind::IllegalValueKind;

/// 对应 `CallingConvention.Type`：调用类型标记接口。
pub trait Type {}

/// 对应 `class CallingConvention`。
pub struct CallingConvention {
    stack_size: i32,
    return_location: Box<dyn AllocatableValue>,
    argument_locations: Vec<Box<dyn AllocatableValue>>,
}

impl CallingConvention {
    /// 对应 `CallingConvention(int stackSize, AllocatableValue returnLocation,
    /// AllocatableValue... argumentLocations)`。
    pub fn new(
        stack_size: i32,
        return_location: Box<dyn AllocatableValue>,
        argument_locations: Vec<Box<dyn AllocatableValue>>,
    ) -> Self {
        Self {
            stack_size,
            return_location,
            argument_locations,
        }
    }

    /// 对应 `getReturn()`。
    pub fn get_return(&self) -> &dyn AllocatableValue {
        self.return_location.as_ref()
    }

    /// 对应 `getArgument(int index)`。
    pub fn get_argument(&self, index: usize) -> &dyn AllocatableValue {
        self.argument_locations[index].as_ref()
    }

    /// 对应 `getStackSize()`。
    pub fn get_stack_size(&self) -> i32 {
        self.stack_size
    }

    /// 对应 `getArgumentCount()`。
    pub fn get_argument_count(&self) -> usize {
        self.argument_locations.len()
    }

    /// 对应 `getArguments()`。
    pub fn get_arguments(&self) -> &[Box<dyn AllocatableValue>] {
        &self.argument_locations
    }

    /// 对应 `toString` 中 `!returnLocation.equals(Value.ILLEGAL)` 判别。`IllegalValue.equals`
    /// 按类型判等，等价于 kind 为 `IllegalValueKind`。
    fn return_is_illegal(&self) -> bool {
        self.return_location
            .get_value_kind()
            .as_any()
            .is::<IllegalValueKind>()
    }
}

impl fmt::Display for CallingConvention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString()`：`CallingConvention[arg0, arg1, ... -> return]`（return 非 ILLEGAL 时）。
        f.write_str("CallingConvention[")?;
        let mut sep = "";
        for op in &self.argument_locations {
            write!(f, "{}{}", sep, op)?;
            sep = ", ";
        }
        if !self.return_is_illegal() {
            write!(f, " -> {}", self.return_location)?;
        }
        f.write_str("]")
    }
}

impl fmt::Debug for CallingConvention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CallingConvention")
            .field("stack_size", &self.stack_size)
            .field("return_location", &self.return_location)
            .field("argument_locations", &self.argument_locations)
            .finish()
    }
}
