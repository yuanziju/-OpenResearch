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

//! 镜像 `jdk.graal.compiler.nodes.PiNode`：类型窄化/守卫节点。
//!
//! 偏离记录：Java `PiNode` 继承 `FloatingGuardedNode` 并实现 `Canonicalizable`。
//! Rust 侧为具体 struct。

use rustci_vm_ci::meta::java_kind::JavaKind;

use crate::nodes::guarding_node::GuardingNode;
use crate::nodes::value_node::ValueNode;

/// 对应 `PiNode.IntrinsifyOp` 枚举：内置窄化操作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntrinsifyOp {
    PositiveInt,
    IntNonZero,
    LongNonZero,
    NonNull,
    FloatNonNan,
    DoubleNonNan,
}

/// 对应 `class PiNode extends FloatingGuardedNode implements Canonicalizable`。
///
/// PiNode 在守卫条件下窄化值的 stamp，用于编码类型信息。
/// 例如，在 instanceof 检查后，PiNode 可以窄化对象的类型。
#[derive(Debug)]
pub struct PiNode {
    /// 对应 `object()`：被窄化的原始值。
    pub object: Box<dyn ValueNode>,
    /// 对应 `guard`：守卫此 PiNode 的节点。
    pub guard: Box<dyn GuardingNode>,
    /// 对应 `piStamp`：窄化后的 stamp 信息。
    /// 在 Rust 侧用 target_kind 简化表示。
    pub target_kind: JavaKind,
    /// 对应 `intrinsifyOp`：内置窄化操作（可选）。
    pub intrinsify_op: Option<IntrinsifyOp>,
}

impl PiNode {
    /// 创建一个新的 PiNode。
    pub fn new(
        object: Box<dyn ValueNode>,
        guard: Box<dyn GuardingNode>,
        target_kind: JavaKind,
    ) -> Self {
        PiNode {
            object,
            guard,
            target_kind,
            intrinsify_op: None,
        }
    }

    /// 创建一个带有内置操作的 PiNode。
    pub fn new_intrinsified(
        object: Box<dyn ValueNode>,
        guard: Box<dyn GuardingNode>,
        op: IntrinsifyOp,
    ) -> Self {
        PiNode {
            object,
            guard,
            target_kind: JavaKind::Object,
            intrinsify_op: Some(op),
        }
    }

    /// 对应 `object()`：获取原始值。
    pub fn object(&self) -> &dyn ValueNode {
        self.object.as_ref()
    }

    /// 对应 `getGuard()`：获取守卫节点。
    pub fn get_guard(&self) -> &dyn GuardingNode {
        self.guard.as_ref()
    }

    /// 对应 `setOriginalNode(ValueNode)`：设置原始节点。
    pub fn set_original_node(&mut self, new_node: Box<dyn ValueNode>) {
        self.object = new_node;
    }

    /// 对应 `getOriginalNode()`：获取原始节点。
    pub fn get_original_node(&self) -> &dyn ValueNode {
        self.object()
    }
}

impl ValueNode for PiNode {
    fn get_stack_kind(&self) -> JavaKind {
        self.target_kind.get_stack_kind()
    }
}
