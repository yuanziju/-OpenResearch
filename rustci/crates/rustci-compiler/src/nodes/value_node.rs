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

//! 镜像 `jdk.graal.compiler.nodes.ValueNode`：图中所有可产生值的节点之基类。
//!
//! 偏离记录：Java `ValueNode` 继承 `Node` 并实现 `ValueNodeInterface`，依赖 `Stamp`、
//! `NodeClass`、`Graph` 等重型框架。Rust 侧将其简化为 trait，保留核心语义：
//! stamp/stamp_kind、is_constant、as_constant 等查询接口。

use rustci_vm_ci::meta::java_kind::JavaKind;
use std::fmt::Debug;

/// 对应 `abstract class ValueNode extends Node implements ValueNodeInterface`。
///
/// 图中所有可产生值的节点（包括局部变量、phi、所有指令）的抽象基类。
pub trait ValueNode: Debug {
    /// 对应 `getStackKind()`：返回本值的栈种类（stack kind）。
    fn get_stack_kind(&self) -> JavaKind;

    /// 对应 `isConstant()`：检查本值是否为常量。
    fn is_constant(&self) -> bool {
        false
    }

    /// 对应 `isNullConstant()`：检查本值是否表示 null 常量。
    fn is_null_constant(&self) -> bool {
        false
    }

    /// 对应 `isDefaultConstant()`：检查本值是否为对应种类的默认常量。
    fn is_default_constant(&self) -> bool {
        false
    }

    /// 对应 `asConstant()`：如果本节点是常量，返回常量值。
    fn as_constant(&self) -> Option<&dyn rustci_vm_ci::meta::java_constant::JavaConstant> {
        None
    }

    /// 对应 `asJavaConstant()`：如果本节点是常量，返回 JavaConstant。
    fn as_java_constant(
        &self,
    ) -> Option<&dyn rustci_vm_ci::meta::java_constant::JavaConstant> {
        self.as_constant()
    }

    /// 对应 `asBool()`：获取布尔值（仅当为常量布尔时有效）。
    fn as_bool(&self) -> Option<bool> {
        self.as_constant().map(|c| c.as_boolean())
    }

    /// 对应 `stamp()`：返回 stamp。
    fn stamp(&self) -> Option<&dyn std::any::Any> {
        None
    }

    /// 对应 `getStableDimension()`：获取稳定维度。
    fn get_stable_dimension(&self) -> i32 {
        0
    }

    /// 对应 `isDefaultStable()`：默认元素是否稳定。
    fn is_default_stable(&self) -> bool {
        false
    }

    /// 对应 `isArrayLength()`：是否为数组长度。
    fn is_array_length(&self) -> bool {
        false
    }

    /// 对应 `inferStamp()`：当输入变化时重新计算 stamp。
    /// 返回 true 表示 stamp 已变化。
    fn infer_stamp(&mut self) -> bool {
        false
    }
}
