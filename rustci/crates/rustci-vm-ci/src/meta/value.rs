// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2024, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.Value`（`abstract class`）：所有值的抽象基类。
//!
//! 偏离记录：
//! - Java `abstract class Value`（带 `valueKind` 字段、`getKindSuffix`/`getValueKind`/
//!   `getPlatformKind`/`identityEquals`/`equals`/`hashCode`）→ Rust `trait Value`。Rust 无继承，
//!   `valueKind` 字段下沉到各实现 struct（`RegisterValue`/`StackSlot`/`IllegalValue` 等），
//!   经 `get_value_kind()` 暴露。`AllocatableValue` 为 `trait AllocatableValue: Value + JavaValue`。
//! - `equals`/`hashCode`（Java `final`，按 `valueKind.equals`/`hashCode`）→ `value_equals`/
//!   `value_hash` 默认方法。`identityEquals`（Java `this == other`）→ `identity_equals`，用
//!   `ptr::eq` 比较 `as_any` 数据指针。
//! - `getValueKind(Class<K> cls)` 泛型版（Java 用反射 `cls.cast`）→ Rust `get_value_kind_as::<K>()`
//!   下转，返回 `Option<&K>`（无法强制转换时返回 `None`，对齐 Java `ClassCastException` 语义的
//!   非抛错变体）。
//! - `ILLEGAL` 常量：Java 是 `IllegalValue`（`AllocatableValue` 子类）单例。Rust 侧 `IllegalValue`
//!   struct 实现 `AllocatableValue`，`illegal_value() -> Box<dyn AllocatableValue>` 每次返回等价
//!   新实例（`IllegalValue.equals` 按类型判等，多实例等价，对齐 Java 注释「may exist multiple
//!   times」语义）。
//! - `NO_VALUES` 常量：`pub const NO_VALUES: &[&dyn Value] = &[]`。

use std::any::Any;
use std::fmt::{Debug, Display};

use crate::meta::allocatable_value::AllocatableValue;
use crate::meta::java_value::JavaValue;
use crate::meta::platform_kind::PlatformKind;
use crate::meta::value_kind::{IllegalValueKind, ValueKind};

/// 对应 `Value.NO_VALUES`。
pub const NO_VALUES: &[&dyn Value] = &[];

/// 对应 `abstract class Value`。
///
/// 增设 `Display` 超 trait：Java `Value` 继承 `Object`（含 `toString`），所有子类覆写或继承
/// `toString`；Rust 侧所有 `Value` 实现已提供 `Display`，增设超 trait 使 `dyn Value`/
/// `dyn AllocatableValue` 可经 `{}` 格式化（对齐 `CallingConvention`/`StackLockValue` 等
/// `toString` 调用路径）。
pub trait Value: Debug + Display {
    /// 对应 `getValueKind()`。
    fn get_value_kind(&self) -> &dyn ValueKind;

    /// 对应 `<K extends ValueKind<K>> K getValueKind(Class<K> cls)`。下转失败返回 `None`
    /// （Java 抛 `ClassCastException`，Rust 侧以 `Option` 表达）。
    fn get_value_kind_as<K: ValueKind + 'static>(&self) -> Option<&K>
    where
        Self: Sized,
    {
        self.get_value_kind().as_any().downcast_ref::<K>()
    }

    /// 对应 `getPlatformKind()`。
    fn get_platform_kind(&self) -> &dyn PlatformKind {
        self.get_value_kind().get_platform_kind()
    }

    /// 对应 `protected final String getKindSuffix()`。
    fn get_kind_suffix(&self) -> String {
        self.get_value_kind().get_kind_suffix()
    }

    /// 对应 `public final boolean identityEquals(Value other)`。
    fn identity_equals(&self, other: &dyn Value) -> bool {
        std::ptr::eq(self.as_any(), other.as_any())
    }

    /// 对应 `equals(Object)`（Java `final`，按 `valueKind.equals`）。
    fn value_equals(&self, other: &dyn Value) -> bool {
        self.get_value_kind().kind_equals(other.get_value_kind())
    }

    /// 对应 `hashCode()`（Java `final`，`41 + valueKind.hashCode()`）。
    fn value_hash(&self) -> u64 {
        41u64.wrapping_add(self.get_value_kind().kind_hash())
    }

    /// Rust 增设：支持 `code::ValueUtil` 的 `instanceof` 下转。
    fn as_any(&self) -> &dyn Any;
}

/// 对应 `Value` 内嵌 `private static final class IllegalValue extends AllocatableValue`。
#[derive(Debug, Default)]
pub struct IllegalValue {
    kind: IllegalValueKind,
}

impl IllegalValue {
    pub fn new() -> Self {
        Self {
            kind: IllegalValueKind,
        }
    }
}

impl Display for IllegalValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("-")
    }
}

impl JavaValue for IllegalValue {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Value for IllegalValue {
    fn get_value_kind(&self) -> &dyn ValueKind {
        &self.kind
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    // Java `IllegalValue.equals`：按类型判等（`other instanceof IllegalValue`）。
    fn value_equals(&self, other: &dyn Value) -> bool {
        other.as_any().is::<IllegalValue>()
    }
}

impl AllocatableValue for IllegalValue {}

/// 对应 `Value.ILLEGAL`（`AllocatableValue` 类型的非法值单例）。每次返回等价新实例
/// （`IllegalValue.equals` 按类型判等，与 Java 反序列化可能产生多实例的行为一致）。
pub fn illegal_value() -> Box<dyn AllocatableValue> {
    Box::new(IllegalValue::new())
}
