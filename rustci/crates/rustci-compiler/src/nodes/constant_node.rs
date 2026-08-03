// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2024, 2026, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.graal.compiler.nodes.ConstantNode`：常量节点。
//!
//! 偏离记录：Java `ConstantNode` 继承 `FloatingNode`，含 `Constant` 值和 `Stamp`。
//! Rust 侧为具体 struct，值用 `Box<dyn JavaConstant>` 表示。

use rustci_vm_ci::meta::java_constant::JavaConstant;
use rustci_vm_ci::meta::java_kind::JavaKind;

use crate::nodes::value_node::ValueNode;

/// 对应 `final class ConstantNode extends FloatingNode implements LIRLowerable, ArrayLengthProvider`。
///
/// 表示编译时常量值。
pub struct ConstantNode {
    /// 对应 `value`：常量值。
    pub value: Box<dyn JavaConstant>,
    /// 对应 `stableDimension`：稳定数组维度，0 表示非稳定数组。
    pub stable_dimension: u32,
    /// 对应 `isDefaultStable`：稳定数组的默认元素是否稳定。
    pub is_default_stable: bool,
}

impl std::fmt::Debug for ConstantNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConstantNode")
            .field("value", &self.value.to_value_string())
            .field("stable_dimension", &self.stable_dimension)
            .field("is_default_stable", &self.is_default_stable)
            .finish()
    }
}

impl ConstantNode {
    /// 创建一个新的 ConstantNode。
    pub fn new(value: Box<dyn JavaConstant>) -> Self {
        ConstantNode {
            value,
            stable_dimension: 0,
            is_default_stable: false,
        }
    }

    /// 创建一个稳定数组的 ConstantNode。
    pub fn new_stable(
        value: Box<dyn JavaConstant>,
        stable_dimension: u32,
        is_default_stable: bool,
    ) -> Self {
        ConstantNode {
            value,
            stable_dimension,
            is_default_stable,
        }
    }

    /// 对应 `getValue()`：获取常量值。
    pub fn get_value(&self) -> &dyn JavaConstant {
        self.value.as_ref()
    }

    /// 对应 `getStableDimension()`：获取稳定数组维度。
    pub fn get_stable_dimension(&self) -> u32 {
        self.stable_dimension
    }

    /// 对应 `isDefaultStable()`：默认元素是否稳定。
    pub fn is_default_stable(&self) -> bool {
        self.is_default_stable
    }

    /// 创建 int 常量。
    pub fn for_int(value: i32) -> Self {
        ConstantNode::new(Box::new(rustci_vm_ci::meta::java_constant::for_int(value)))
    }

    /// 创建 long 常量。
    pub fn for_long(value: i64) -> Self {
        ConstantNode::new(Box::new(rustci_vm_ci::meta::java_constant::for_long(value)))
    }

    /// 创建 float 常量。
    pub fn for_float(value: f32) -> Self {
        ConstantNode::new(Box::new(rustci_vm_ci::meta::java_constant::for_float(
            value,
        )))
    }

    /// 创建 double 常量。
    pub fn for_double(value: f64) -> Self {
        ConstantNode::new(Box::new(rustci_vm_ci::meta::java_constant::for_double(
            value,
        )))
    }

    /// 创建 boolean 常量。
    pub fn for_boolean(value: bool) -> Self {
        ConstantNode::new(Box::new(rustci_vm_ci::meta::java_constant::for_boolean(
            value,
        )))
    }

    /// 创建 null 对象常量。
    pub fn for_null() -> Self {
        ConstantNode::new(Box::new(rustci_vm_ci::meta::java_constant::null_pointer()))
    }

    /// 创建对应 JavaKind 的默认常量。
    pub fn default_for_kind(kind: JavaKind) -> Option<Self> {
        match kind {
            JavaKind::Boolean
            | JavaKind::Byte
            | JavaKind::Char
            | JavaKind::Short
            | JavaKind::Int => Some(ConstantNode::for_int(0)),
            JavaKind::Long => Some(ConstantNode::for_long(0)),
            JavaKind::Float => Some(ConstantNode::for_float(0.0)),
            JavaKind::Double => Some(ConstantNode::for_double(0.0)),
            JavaKind::Object => Some(ConstantNode::for_null()),
            _ => None,
        }
    }

    /// 对应 `isArrayLength()`：检查是否为数组长度。
    pub fn is_array_length(&self) -> bool {
        self.value.as_boolean()
    }

    /// 对应 `length()`：获取数组长度（来自 ArrayLengthProvider）。
    pub fn length(&self) -> i32 {
        // In the full implementation, this returns the array length for boxed array constants.
        self.get_value().as_int()
    }
}

impl ValueNode for ConstantNode {
    fn get_stack_kind(&self) -> JavaKind {
        self.value.get_java_kind().get_stack_kind()
    }

    fn is_constant(&self) -> bool {
        true
    }

    fn is_null_constant(&self) -> bool {
        self.value.is_null()
    }

    fn is_default_constant(&self) -> bool {
        self.value.is_default_for_kind()
    }

    fn as_constant(&self) -> Option<&dyn JavaConstant> {
        Some(self.value.as_ref())
    }

    fn is_array_length(&self) -> bool {
        self.is_array_length()
    }
}
