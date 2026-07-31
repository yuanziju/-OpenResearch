// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2024, Oracle and/or its affiliates. All rights reserved.
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

//! Rust 镜像 `java.lang.Class` 与 `java.lang.reflect.*` 抽象。
//!
//! 偏离记录：Java 侧 `Class<?>`/`Annotation`/`Type`/`Executable`/`Method`/
//! `Constructor`/`Field` 均为 JDK 反射类型，Rust 侧无直接对应。本期 T4 为接口镜像，
//! 不绑定具体 JVM 反射对象；这些类型作为不透明标记（marker）出现于方法签名中，
//! 具体绑定延至 T7（hotspot 层）。`JavaClass` 携带 `&'static str` 名称以保留
//! `JavaKind.isPrimitive`/`fromJavaClass` 等方法的区分行为（对应 Java 侧
//! 各 primitive Class 对象的可区分性）。

use std::cmp::Ordering;

/// 镜像 `java.lang.Class<?>`。携带类名（`Class.getName()` 形式）以区分 primitive 类对象。
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct JavaClass {
    name: &'static str,
}

impl JavaClass {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

/// 镜像 `java.lang.annotation.Annotation`。不透明标记 trait，T7 提供具体绑定。
pub trait JavaAnnotation: std::any::Any {}

/// 镜像 `java.lang.reflect.Type`。不透明标记 trait，T7 提供具体绑定。
pub trait JavaReflectType: std::any::Any {}

/// 镜像 `java.lang.reflect.Executable`。不透明标记 trait，T7 提供具体绑定。
pub trait JavaExecutable: std::any::Any {}

/// 镜像 `java.lang.reflect.Method`。不透明标记 trait，T7 提供具体绑定。
pub trait JavaReflectMethod: std::any::Any {}

/// 镜像 `java.lang.reflect.Constructor`。不透明标记 trait，T7 提供具体绑定。
pub trait JavaReflectConstructor: std::any::Any {}

/// 镜像 `java.lang.reflect.Field`。不透明标记 trait，T7 提供具体绑定。
pub trait JavaReflectField: std::any::Any {}

/// 镜像 `java.lang.invoke.MethodHandle`。不透明标记 trait，T7 提供具体绑定。
pub trait JavaMethodHandle: std::any::Any {}

/// 镜像 `java.lang.invoke.MethodType`。不透明标记 trait，T7 提供具体绑定。
pub trait JavaMethodType: std::any::Any {}

/// `Comparable` 风格辅助：反射对象在 T4 无序，统一返回 `Equal`，仅满足签名需要。
pub fn reflect_eq<T: ?Sized>(_a: &T, _b: &T) -> bool {
    false
}

#[allow(dead_code)]
pub fn reflect_cmp<T: ?Sized>(_a: &T, _b: &T) -> Ordering {
    Ordering::Equal
}
