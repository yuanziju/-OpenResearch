// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2019, 2024, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.  Oracle designates this
 * particular file as subject to the "Classpath" exception as provided
 * by Oracle in the LICENSE file that accompanied this code.
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

//! 镜像 `jdk.graal.compiler.lir.LIRValueClass`：LIR 值的分类枚举。
//!
//! 偏离记录：Java `enum LIRValueClass` → Rust enum。

use std::fmt;

/// 对应 `enum LIRValueClass`。
///
/// 描述 LIR 值的运行时类：非法值、Java 原始值、对象引用、地址等。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LIRValueClass {
    /// 非法值。
    Illegal,
    /// Java 原始值（int, long, float, double 等）。
    JavaPrimitive,
    /// Java 对象引用。
    JavaReference,
    /// 地址值（指针）。
    Address,
    /// 未知类型。
    Unknown,
}

impl LIRValueClass {
    /// 对应 `getLength()`：该值类占用的寄存器槽数。
    pub fn get_length(&self) -> usize {
        match self {
            LIRValueClass::Illegal | LIRValueClass::Unknown => 0,
            LIRValueClass::JavaPrimitive | LIRValueClass::JavaReference | LIRValueClass::Address => 1,
        }
    }
}

impl fmt::Display for LIRValueClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LIRValueClass::Illegal => write!(f, "Illegal"),
            LIRValueClass::JavaPrimitive => write!(f, "JavaPrimitive"),
            LIRValueClass::JavaReference => write!(f, "JavaReference"),
            LIRValueClass::Address => write!(f, "Address"),
            LIRValueClass::Unknown => write!(f, "Unknown"),
        }
    }
}