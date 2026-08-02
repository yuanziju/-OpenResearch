// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2016, 2024, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.ValueKind<K extends ValueKind<K>>`：值的类型抽象基类。
//!
//! 偏离记录：
//! - Java 自递归泛型 `K extends ValueKind<K>` → Rust 非泛型 `trait ValueKind`（自递归泛型在
//!   Rust 无等价物，降级为非泛型 trait；`changeType` 返回 `Box<dyn ValueKind>` 而非自类型 `K`）。
//! - 增设 `Debug`/`Display` 超 trait（`Value.getKindSuffix` 默认委托 `toString`，trait 对象需
//!   `Display`；容器持 `Box<dyn ValueKind>` 需 `Debug`）。增设 `as_any`/`kind_equals`/`kind_hash`
//!   以支持 `Value` trait 对象的相等/哈希与下转（Java `Value.equals`/`hashCode` 委托
//!   `valueKind.equals`/`hashCode`，Rust trait 对象需显式方法）。
//! - `Illegal` 常量：Java 是 `IllegalValueKind` 单例（`static final`）。Rust 侧 `IllegalValueKind`
//!   为单元 struct，提供 `pub const ILLEGAL` 与 `illegal_value_kind() -> Box<dyn ValueKind>`。
//!   `IllegalValueKind.kind_equals` 按类型判等（对齐 Java 单例语义，多实例等价）。
//! - 嵌套 `private enum IllegalKind implements PlatformKind` → 同文件 `pub enum IllegalKind`，
//!   单例经 `static` 暴露，`get_key` 经 `OnceLock<EnumKey<IllegalKind>>` 惰性构造。

use std::any::Any;
use std::fmt::{Debug, Display};
use std::sync::OnceLock;

use crate::meta::platform_kind::{EnumKey, Key, PlatformKind};

/// 对应 `abstract class ValueKind<K extends ValueKind<K>>`。
pub trait ValueKind: Debug + Display {
    /// 对应 `getPlatformKind()`。
    fn get_platform_kind(&self) -> &dyn PlatformKind;

    /// 对应 `changeType(PlatformKind)`。子类须覆写以保留额外信息。
    fn change_type(&self, new_platform_kind: &dyn PlatformKind) -> Box<dyn ValueKind>;

    /// 对应 `getKindSuffix()`，默认委托 `toString`。
    fn get_kind_suffix(&self) -> String {
        self.to_string()
    }

    /// Rust 增设：支持 `Value` trait 对象的 `equals` 下转与判等。
    fn as_any(&self) -> &dyn Any;

    /// Rust 增设：对应 Java `ValueKind` 对象的引用复制（Java 侧 `getValueKind()` 返回引用，
    /// 传入 `StackSlot.get(kind, ...)` 等构造器仅复制引用）。Rust 侧 `Box<dyn ValueKind>`
    /// 无 `Clone`，故增设 `clone_box` 供 `StackSlot::as_out_arg`/`as_in_arg` 等需复制 kind
    /// 的场景使用。各实现返回等值新 Box（`IllegalValueKind` 为单元 struct，等价新实例）。
    fn clone_box(&self) -> Box<dyn ValueKind>;

    /// Rust 增设：对应 Java `ValueKind.equals`（默认 `Object.equals` 即引用相等）。
    fn kind_equals(&self, other: &dyn ValueKind) -> bool {
        std::ptr::eq(self.as_any(), other.as_any())
    }

    /// Rust 增设：对应 Java `ValueKind.hashCode`（默认 `Object.hashCode`）。
    fn kind_hash(&self) -> u64 {
        0
    }
}

/// 对应 `ValueKind` 内嵌 `private enum IllegalKind implements PlatformKind`。
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum IllegalKind {
    Illegal,
}

static ILLEGAL_INSTANCE: IllegalKind = IllegalKind::Illegal;
static ILLEGAL_KEY: OnceLock<EnumKey<IllegalKind>> = OnceLock::new();

impl PlatformKind for IllegalKind {
    fn name(&self) -> &str {
        "ILLEGAL"
    }

    fn get_key(&self) -> &dyn Key {
        ILLEGAL_KEY.get_or_init(|| EnumKey::new(&ILLEGAL_INSTANCE, 0))
    }

    fn get_size_in_bytes(&self) -> i32 {
        0
    }

    fn get_vector_length(&self) -> i32 {
        0
    }

    fn get_type_char(&self) -> char {
        '-'
    }
}

/// 对应 `ValueKind` 内嵌 `private static class IllegalValueKind extends ValueKind<IllegalValueKind>`。
#[derive(Debug, Clone, Copy, Default)]
pub struct IllegalValueKind;

impl IllegalValueKind {
    pub fn new() -> Self {
        Self
    }
}

impl Display for IllegalValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ILLEGAL")
    }
}

impl ValueKind for IllegalValueKind {
    fn get_platform_kind(&self) -> &dyn PlatformKind {
        &ILLEGAL_INSTANCE
    }

    fn change_type(&self, _new_platform_kind: &dyn PlatformKind) -> Box<dyn ValueKind> {
        // Java `IllegalValueKind.changeType` 返回 `this`；单元 struct 等价新实例。
        Box::new(IllegalValueKind)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn ValueKind> {
        Box::new(IllegalValueKind)
    }

    fn kind_equals(&self, other: &dyn ValueKind) -> bool {
        other.as_any().is::<IllegalValueKind>()
    }

    fn kind_hash(&self) -> u64 {
        // 单例语义：所有 `IllegalValueKind` 等价，固定哈希。
        0x494c4c4547414cu64
    }
}

/// 对应 `ValueKind.Illegal` 静态字段（`IllegalValueKind` 单例）。
pub const ILLEGAL: IllegalValueKind = IllegalValueKind;

/// 取 `Illegal` 的 `Box<dyn ValueKind>`（对应 Java 把 `ValueKind.Illegal` 赋给 `ValueKind<?>` 字段）。
pub fn illegal_value_kind() -> Box<dyn ValueKind> {
    Box::new(IllegalValueKind)
}
