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

//! 镜像 `jdk.graal.compiler.lir.BlockValue`：块值。
//!
//! 偏离记录：Java `final class BlockValue` → Rust struct。
//! 块值表示一个值，其定义在另一个基本块中（phi 函数或跨块引用）。

use std::any::Any;
use std::fmt;

use rustci_vm_ci::meta::value::Value;
use rustci_vm_ci::meta::value_kind::ValueKind;

use crate::lir::label_ref::LabelRef;

/// 对应 `public final class BlockValue`。
///
/// 块值，表示一个值来自另一个基本块。用于 phi 函数或跨块值引用。
pub struct BlockValue {
    /// 对应 `kind`：值的种类。
    kind: Box<dyn ValueKind>,
    /// 对应 `label`：来源基本块标签。
    label: LabelRef,
    /// 对应 `value`：该块中的值。
    value: Option<Box<dyn Value>>,
    /// 对应 `index`：在 phi 中的索引。
    index: usize,
}

impl Clone for BlockValue {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone_box(),
            label: self.label,
            value: self.value.as_ref().map(|v| v.clone_box()),
            index: self.index,
        }
    }
}

impl BlockValue {
    /// 创建块值。
    pub fn new(kind: Box<dyn ValueKind>, label: LabelRef, index: usize) -> Self {
        Self {
            kind,
            label,
            value: None,
            index,
        }
    }

    /// 对应 `getLabel()`：获取来源基本块标签。
    pub fn get_label(&self) -> LabelRef {
        self.label
    }

    /// 对应 `getValue()`：获取该块中的值。
    pub fn get_value(&self) -> Option<&dyn Value> {
        self.value.as_ref().map(|v| v.as_ref())
    }

    /// 设置该块中的值。
    pub fn set_value(&mut self, value: Box<dyn Value>) {
        self.value = Some(value);
    }

    /// 对应 `getIndex()`：获取在 phi 中的索引。
    pub fn get_index(&self) -> usize {
        self.index
    }
}

impl fmt::Debug for BlockValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BlockValue")
            .field("label", &self.label)
            .field("index", &self.index)
            .field("kind", &self.kind)
            .finish()
    }
}

impl fmt::Display for BlockValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BlockValue[{},{}]{}", self.label.get_block_index(), self.index, self.get_kind_suffix())
    }
}

impl Value for BlockValue {
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