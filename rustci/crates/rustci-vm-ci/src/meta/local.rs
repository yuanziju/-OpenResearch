// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2011, 2024, Oracle and/or its affiliates. All rights reserved.
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
 * or visit www.oracle if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.meta.Local`：局部变量在 BCI 区间内的存活描述。
//!
//! 偏离记录：Java `Local.equals` 调 `this.type.equals(that.type)`；`JavaType` 无 `equals`
//! 覆写（HotSpot `ResolvedJavaType.equals` 为引用相等，`UnresolvedJavaType.equals` 为名相等）。
//! Rust trait 对象无法表达该多态 `Object.equals`，`type` 字段相等性退化为指针相等。
//! `hashCode` 走 `super.hashCode()`（即 `Object.hashCode`，identity）→ Rust 用指针地址哈希。

use std::fmt;
use std::hash::{Hash, Hasher};

use crate::meta::java_type::JavaType;

/// 对应 `public class Local`。
pub struct Local {
    name: String,
    start_bci: i32,
    end_bci: i32,
    slot: i32,
    r#type: Box<dyn JavaType>,
}

impl Local {
    /// 对应 `Local(String name, JavaType type, int startBci, int endBci, int slot)`。
    pub fn new(
        name: impl Into<String>,
        r#type: Box<dyn JavaType>,
        start_bci: i32,
        end_bci: i32,
        slot: i32,
    ) -> Self {
        Self {
            name: name.into(),
            start_bci,
            end_bci,
            slot,
            r#type,
        }
    }

    /// 对应 `getStartBCI()`。
    pub fn get_start_bci(&self) -> i32 {
        self.start_bci
    }

    /// 对应 `getEndBCI()`。
    pub fn get_end_bci(&self) -> i32 {
        self.end_bci
    }

    /// 对应 `getName()`。
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// 对应 `getType()`：返回 `&dyn JavaType`（Java 返回 `JavaType`）。
    pub fn get_type(&self) -> &dyn JavaType {
        self.r#type.as_ref()
    }

    /// 对应 `getSlot()`。
    pub fn get_slot(&self) -> i32 {
        self.slot
    }
}

impl fmt::Debug for Local {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Local")
            .field("name", &self.name)
            .field("type", &self.r#type)
            .field("startBci", &self.start_bci)
            .field("endBci", &self.end_bci)
            .field("slot", &self.slot)
            .finish()
    }
}

impl fmt::Display for Local {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString()`。Java `type` 经 `%s` 调 `toString()`（`JavaType` 无覆写即
        // `Object.toString`）；Rust 侧 `dyn JavaType` 无 `Display`，改用 `to_java_name()`
        // 作可读近似（偏离：输出 Java 名而非 `type@hash`，同 `ExceptionHandler` 偏离）。
        write!(
            f,
            "LocalImpl<name={}, type={}, startBci={}, endBci={}, slot={}>",
            self.name,
            self.r#type.to_java_name(),
            self.start_bci,
            self.end_bci,
            self.slot
        )
    }
}

impl PartialEq for Local {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.start_bci == other.start_bci
            && self.end_bci == other.end_bci
            && self.slot == other.slot
            && std::ptr::eq(&*self.r#type, &*other.r#type)
    }
}

impl Eq for Local {}

impl Hash for Local {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 对应 `super.hashCode()`（identity hash）；Rust 用指针地址占位。
        let addr = self as *const Self as usize;
        (addr as u64).hash(state);
    }
}
