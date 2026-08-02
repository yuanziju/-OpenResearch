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
 * version 2 for more details (a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Packaging, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.code.ValueKindFactory<K extends ValueKind<K>>`：编译器创建自定义 `ValueKind` 的工厂。
//!
//! 偏离记录：Java 泛型 `K extends ValueKind<K>` → Rust 非泛型 `trait ValueKindFactory`，
//! `getValueKind(JavaKind)` 返回 `Box<dyn ValueKind>` 而非具体类型 `K`。原因：
//! `RegisterConfig::getCallingConvention` 经 `dyn RegisterConfig` trait 对象调用，须接受
//! 任意 `ValueKindFactory` 实现；关联类型 `type K: ValueKind` 使 trait 不可作 `dyn`，
//! 故降级为类型擦除的 `Box<dyn ValueKind>` 返回（对齐 Java `ValueKindFactory<?>` 通配符语义）。

use crate::meta::java_kind::JavaKind;
use crate::meta::value_kind::ValueKind;

/// 对应 `interface ValueKindFactory<K extends ValueKind<K>>`。
pub trait ValueKindFactory {
    /// 对应 `getValueKind(JavaKind)`。返回 `Box<dyn ValueKind>`（类型擦除，对齐 `ValueKindFactory<?>`）。
    fn get_value_kind(&self, java_kind: JavaKind) -> Box<dyn ValueKind>;
}
