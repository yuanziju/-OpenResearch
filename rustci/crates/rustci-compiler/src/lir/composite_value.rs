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

//! 镜像 `jdk.graal.compiler.lir.CompositeValue`：复合值。
//!
//! 偏离记录：Java `abstract class CompositeValue implements Value` → Rust struct。
//! 复合值由多个部分值组成，分布在多个寄存器/栈槽上。

use std::any::Any;
use std::fmt;

use rustci_vm_ci::meta::value::Value;
use rustci_vm_ci::meta::value_kind::ValueKind;

/// 对应 `abstract class CompositeValue implements Value`。
///
/// 复合值，表示由多个寄存器/栈槽组合而成的值。
/// 例如，在 32 位平台上存储 64 位 long 值需要两个寄存器。
pub struct CompositeValue {
    /// 对应 `kind`：值的种类。
    kind: Box<dyn ValueKind>,
    /// 对应 `parts`：组成该复合值的各部分值。
    parts: Vec<Box<dyn Value>>,
}

impl Clone for CompositeValue {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone_box(),
            parts: self.parts.iter().map(|v| v.clone_box()).collect(),
        }
    }
}

impl CompositeValue {
    /// 创建复合值。
    pub fn new(kind: Box<dyn ValueKind>, parts: Vec<Box<dyn Value>>) -> Self {
        Self { kind, parts }
    }

    /// 对应 `getParts()`：获取组成部分值。
    pub fn get_parts(&self) -> &[Box<dyn Value>] {
        &self.parts
    }

    /// 获取可变组成部分值。
    pub fn get_parts_mut(&mut self) -> &mut Vec<Box<dyn Value>> {
        &mut self.parts
    }

    /// 获取组成部分数量。
    pub fn get_part_count(&self) -> usize {
        self.parts.len()
    }

    /// 获取指定索引的组成部分值。
    pub fn get_part(&self, index: usize) -> Option<&dyn Value> {
        self.parts.get(index).map(|v| v.as_ref())
    }
}

impl fmt::Debug for CompositeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompositeValue")
            .field("kind", &self.kind)
            .field("parts", &self.parts)
            .finish()
    }
}

impl fmt::Display for CompositeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Composite{}", self.get_kind_suffix())
    }
}

impl Value for CompositeValue {
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