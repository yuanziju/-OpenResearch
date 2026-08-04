// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2013, 2015, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.PlatformKind`：平台相关的低层值种类。
//!
//! 偏离记录：Java 嵌套 `interface Key` → Rust trait `Key`；`class EnumKey<E extends Enum<E>>`
//! 的泛型 `E extends Enum<E>` → Rust `E: 'static`（Rust 无 Java 枚举自类型约束，用 `'static`
//! 保留可作 map key 的稳定语义）。`EnumKey` 持有 `E` 引用而非 `Enum<E>`，按 `e.ordinal()`
//! （由 `EnumKey::ordinal()` 提供）做 hash/equals。

use std::hash::{Hash, Hasher};

/// 对应 `PlatformKind.Key`：可作为 map 稳定键的标记接口。
pub trait Key {}

/// 对应 `PlatformKind.EnumKey<E extends Enum<E>>`。
pub struct EnumKey<E: 'static> {
    e: &'static E,
    ordinal: i32,
}

impl<E: 'static> EnumKey<E> {
    pub fn new(e: &'static E, ordinal: i32) -> Self {
        Self { e, ordinal }
    }
}

impl<E: 'static> Key for EnumKey<E> {}

impl<E: 'static> Hash for EnumKey<E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ordinal.hash(state);
    }
}

impl<E: 'static> PartialEq for EnumKey<E> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.e as *const E, other.e as *const E)
    }
}

impl<E: 'static> Eq for EnumKey<E> {}

/// 对应 `interface PlatformKind`。
pub trait PlatformKind {
    fn name(&self) -> &str;
    fn get_key(&self) -> &dyn Key;
    fn get_size_in_bytes(&self) -> i32;
    fn get_vector_length(&self) -> i32;
    fn get_type_char(&self) -> char;

    /// Rust 增设：支持 `ValueKind::change_type` / `LIRKind::combine` 等需要复制
    /// PlatformKind 的场景。各实现返回等值新 Box。
    fn clone_box(&self) -> Box<dyn PlatformKind>;
}
