// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2014, 2015, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.Constant`：编译期/运行时常量值的根接口。
//!
//! 偏离记录：Java 通用 `Object.toString` → Rust `Display` 超 trait（`Assumptions.CallSiteTargetValue.toString`
//! 等默认方法需拼接 `Constant` trait 对象）。增设 `as_any` 以支持 `JavaConstant.isNull(Constant)`
//! 静态方法的 `instanceof` 下转（Java 用 `instanceof`，Rust 用 `Any` 下转）。增设 `Debug`
//! 超 trait：`ConstantPoolEntry` 等容器持 `Box<dyn Constant>` 需 `Debug` 派生（Java 侧
//! `Object.toString` 可拼，Rust 侧 trait 对象需 `Debug` 超 trait 才能派生）。

use std::fmt::{Debug, Display};

/// 对应 `interface Constant`。
pub trait Constant: Display + Debug {
    fn is_default_for_kind(&self) -> bool;
    fn to_value_string(&self) -> String;
    /// Rust 增设：支持 `JavaConstant::is_null` 静态方法的 `instanceof` 下转。
    fn as_any(&self) -> &dyn std::any::Any;
}
