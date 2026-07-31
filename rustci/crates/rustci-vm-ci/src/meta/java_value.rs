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
//! 的容器需 `Debug` 派生）；不增设 `as_any`，因 `JavaConstant: Constant + JavaValue` 中
//! `Constant` 已提供 `as_any`，trait 对象下转统一走 `Constant::as_any`（避免双超 trait 同名
//! 方法歧义）。

/// 标记实现：表示一个 Java 值。对应 Java `interface JavaValue`（空标记接口）。
pub trait JavaValue: std::fmt::Debug {}
