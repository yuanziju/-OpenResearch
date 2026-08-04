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

//! 镜像 `jdk.graal.compiler.lir.ConstantValue`：LIR 常量值。
//!
//! 偏离记录：Java `final class ConstantValue extends Value` → Rust struct。
//! 常量值封装一个 Java 常量，在 LIR 中直接使用。

use std::any::Any;
use std::fmt;

use rustci_vm_ci::meta::java_constant::JavaConstant;
use rustci_vm_ci::meta::value::Value;
use rustci_vm_ci::meta::value_kind::ValueKind;

/// 对应 `public final class ConstantValue extends Value`。
///
/// LIR 中的常量值，封装一个 Java 常量。
pub struct ConstantValue {
    /// 对应 `kind`：值的种类。
    kind: Box<dyn ValueKind>,
    /// 对应 `constant`：Java 常量。
    constant: Box<dyn JavaConstant>,
}

impl Clone for ConstantValue {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone_box(),
            constant: self.constant.clone_box(),
        }
    }
}

impl ConstantValue {
    /// 创建常量值。
    pub fn new(kind: Box<dyn ValueKind>, constant: Box<dyn JavaConstant>) -> Self {
        Self { kind, constant }
    }

    /// 对应 `getConstant()`：获取 Java 常量。
    pub fn get_constant(&self) -> &dyn JavaConstant {
        self.constant.as_ref()
    }
}

impl fmt::Debug for ConstantValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConstantValue")
            .field("kind", &self.kind)
            .field("constant", &self.constant.to_value_string())
            .finish()
    }
}

impl fmt::Display for ConstantValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.constant.to_value_string(), self.get_kind_suffix())
    }
}

impl Value for ConstantValue {
    fn get_value_kind(&self) -> &dyn ValueKind {
        self.kind.as_ref()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Value> {
        Box::new(self.clone())
    }
}