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

//! 镜像 `jdk.graal.compiler.lir.Variable`：LIR SSA 变量（虚拟寄存器）。
//!
//! 偏离记录：Java `final class Variable extends AllocatableValue` → Rust struct。
//! 变量是 LIR 中 SSA 形式的虚拟寄存器，具有唯一索引。

use std::any::Any;
use std::fmt;

use rustci_vm_ci::meta::allocatable_value::AllocatableValue;
use rustci_vm_ci::meta::java_value::JavaValue;
use rustci_vm_ci::meta::value::Value;
use rustci_vm_ci::meta::value_kind::ValueKind;

/// 对应 `public final class Variable extends AllocatableValue`。
///
/// LIR 中的 SSA 变量，表示一个虚拟寄存器。每个变量有唯一的索引。
pub struct Variable {
    /// 对应 `kind`：值的种类。
    kind: Box<dyn ValueKind>,
    /// 对应 `index`：变量索引（全局唯一）。
    index: i32,
    /// 对应 `name`：变量名（调试用）。
    name: String,
}

impl Clone for Variable {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone_box(),
            index: self.index,
            name: self.name.clone(),
        }
    }
}

impl Variable {
    /// 对应 `Variable(ValueKind<?>, int)`：按种类和索引创建变量。
    pub fn new(kind: Box<dyn ValueKind>, index: i32) -> Self {
        Self {
            kind,
            index,
            name: format!("v{}", index),
        }
    }

    /// 对应 `Variable(ValueKind<?>, int, String)`：按种类、索引和名称创建变量。
    pub fn new_named(kind: Box<dyn ValueKind>, index: i32, name: &str) -> Self {
        Self {
            kind,
            index,
            name: name.to_string(),
        }
    }

    /// 对应 `getIndex()`：获取变量索引。
    pub fn get_index(&self) -> i32 {
        self.index
    }

    /// 对应 `getName()`：获取变量名称。
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// 设置变量名称。
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Variable")
            .field("index", &self.index)
            .field("name", &self.name)
            .field("kind", &self.kind)
            .finish()
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.name, self.get_kind_suffix())
    }
}

impl Value for Variable {
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

impl JavaValue for Variable {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AllocatableValue for Variable {}