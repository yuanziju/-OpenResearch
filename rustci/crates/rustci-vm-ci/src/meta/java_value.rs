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

//! 镜像 `jdk.vm.ci.meta.JavaValue`：表示 Java 值的标记接口（无方法）。
//!
//! 偏离记录：Java `JavaValue` 为空标记接口。Rust 侧增设 `Debug` 超 trait（持 `Box<dyn JavaValue>`
//! 的容器需 `Debug` 派生）。原 T4 不增设 `as_any`（`JavaConstant` 下转统一走 `Constant::as_any`）；
//! 引入 `code::ValueUtil` 后需对 bare `&dyn JavaValue` 做 `instanceof` 下转（`VirtualObject`/
//! `StackLockValue`/`IllegalValue` 均为 `JavaValue` 但非 `Constant`，`Constant::as_any` 不可达），
//! 故增设 `Any` 超 trait + 必需方法 `as_any`（各实现返回 `self`；`NullConstant`/`PrimitiveConstant`/
//! `RawConstant`/`IllegalValue` 等实现同步补齐）。`Constant::as_any` 与本方法同名，但既有调用均经
//! `&dyn Constant`/`&dyn JavaValue` 单一 trait 对象分派，无歧义。

/// 标记实现：表示一个 Java 值。对应 Java `interface JavaValue`（空标记接口）。
pub trait JavaValue: std::fmt::Debug + std::any::Any {
    /// Rust 增设：支持 `code::ValueUtil` 对 `&dyn JavaValue` 的 `instanceof` 下转。
    /// 各实现返回 `self`（实现类型为 `'static` Sized，可 coerce 到 `&dyn Any`）。
    fn as_any(&self) -> &dyn std::any::Any;
}
