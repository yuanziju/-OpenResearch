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

//! 镜像 `jdk.vm.ci.code.Register`：目标机器寄存器。
//!
//! 偏离记录：
//! - Java `final class Register implements Comparable<Register>` → Rust `pub struct Register`
//!   实现 `Ord`/`PartialOrd`（按 `number`）。Java 嵌套 `class RegisterCategory` → 同文件
//!   `pub struct RegisterCategory`。
//! - Java `String name` 字段 → Rust `&'static str`：寄存器名在 JVMCI 中恒为编译期字面量
//!   （"rax"/"noreg" 等），用 `&'static str` 使 `SPECIAL`/`NONE` 可作 `const`（对齐 Java
//!   `static final` 字段语义）。`RegisterCategory.name` 同理。
//! - `RegisterCategory`/`Register` 均为 `Copy`（无堆字段），按值传递；`equals` 按 `number`
//!   （对齐 Java `Register.equals`）。`RegisterCategory.equals` 按 `name`。
//! - `asValue(ValueKind)` 返回 `RegisterValue`（持 `kind` 与 `Register` 副本）。

use std::cmp::Ordering;
use std::fmt;

use crate::code::register_value::RegisterValue;
use crate::meta::value_kind::{IllegalValueKind, ValueKind};

/// 对应 `Register.RegisterCategory`：平台特定的寄存器类别。
///
/// `PartialEq`/`Hash` 手动实现按 `name`（对齐 Java `RegisterCategory.equals`/`hashCode`，
/// 不含 `mayContainReference`）。
#[derive(Debug, Clone, Copy, Eq)]
pub struct RegisterCategory {
    name: &'static str,
    may_contain_reference: bool,
}

impl PartialEq for RegisterCategory {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::hash::Hash for RegisterCategory {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl RegisterCategory {
    /// 对应 `RegisterCategory(String name)`：默认 `mayContainReference = true`。
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            may_contain_reference: true,
        }
    }

    /// 对应 `RegisterCategory(String name, boolean mayContainReference)`。
    pub fn with_reference(name: &'static str, may_contain_reference: bool) -> Self {
        Self {
            name,
            may_contain_reference,
        }
    }

    /// 对应 `RegisterCategory.toString()`。
    pub fn name(&self) -> &'static str {
        self.name
    }
}

impl fmt::Display for RegisterCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

/// 对应 `Register.SPECIAL`。
pub const SPECIAL: RegisterCategory = RegisterCategory {
    name: "SPECIAL",
    may_contain_reference: true,
};

/// 对应 `final class Register implements Comparable<Register>`。
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Register {
    /// 对应 `public final int number`。
    pub number: i32,
    /// 对应 `public final String name`。
    pub name: &'static str,
    /// 对应 `public final int encoding`。
    pub encoding: i32,
    register_category: RegisterCategory,
}

/// 对应 `Register.None`（非法寄存器）。
pub const NONE: Register = Register {
    number: -1,
    encoding: -1,
    name: "noreg",
    register_category: SPECIAL,
};

impl Register {
    /// 对应 `Register(int number, int encoding, String name, RegisterCategory registerCategory)`。
    pub fn new(
        number: i32,
        encoding: i32,
        name: &'static str,
        register_category: RegisterCategory,
    ) -> Self {
        Self {
            number,
            name,
            encoding,
            register_category,
        }
    }

    /// 对应 `encoding()`。
    pub fn encoding(&self) -> i32 {
        self.encoding
    }

    /// 对应 `getRegisterCategory()`。
    pub fn get_register_category(&self) -> RegisterCategory {
        self.register_category
    }

    /// 对应 `mayContainReference()`。
    pub fn may_contain_reference(&self) -> bool {
        self.register_category.may_contain_reference
    }

    /// 对应 `asValue(ValueKind<?> kind)`。
    pub fn as_value(&self, kind: Box<dyn ValueKind>) -> RegisterValue {
        RegisterValue::new(kind, *self)
    }

    /// 对应 `asValue()`（`ValueKind.Illegal`）。
    pub fn as_value_illegal(&self) -> RegisterValue {
        self.as_value(Box::new(IllegalValueKind))
    }

    /// 对应 `isValid()`。
    pub fn is_valid(&self) -> bool {
        self.number >= 0
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

impl Ord for Register {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialOrd for Register {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
