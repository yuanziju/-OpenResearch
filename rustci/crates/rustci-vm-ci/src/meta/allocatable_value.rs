// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2013, 2024, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.AllocatableValue`（`abstract class extends Value implements JavaValue`）：
//! 由寄存器分配器管理的位置（寄存器/栈槽）的公共基类。
//!
//! 偏离记录：Java `abstract class AllocatableValue extends Value implements JavaValue`
//! （无新增方法）→ Rust `trait AllocatableValue: Value + JavaValue`（空标记 trait）。Rust 无继承，
//! `RegisterValue`/`StackSlot`/`IllegalValue` 各自持 `kind` 字段并 impl `Value` + `AllocatableValue`。
//! `NONE` 常量：`pub const NONE: &[&dyn AllocatableValue] = &[]`。

use crate::meta::java_value::JavaValue;
use crate::meta::value::Value;

/// 对应 `AllocatableValue.NONE`。
pub const NONE: &[&dyn AllocatableValue] = &[];

/// 对应 `abstract class AllocatableValue extends Value implements JavaValue`（空标记）。
pub trait AllocatableValue: Value + JavaValue {}
