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

//! 镜像 `jdk.graal.compiler.nodes.GuardNode`：守卫节点，条件为假时去优化。
//!
//! 偏离记录：Java `GuardNode` 继承 `FloatingAnchoredNode`，依赖 `LogicNode`、`AnchoringNode`、
//! `SpeculationLog.Speculation` 等类型。Rust 侧简化为具体 struct，保留核心字段。

use rustci_vm_ci::meta::deoptimization::{DeoptimizationAction, DeoptimizationReason};

use crate::nodes::guarding_node::GuardingNode;
use crate::nodes::value_node::ValueNode;

/// 对应 `class GuardNode extends FloatingAnchoredNode implements Canonicalizable, GuardingNode, DeoptimizingGuard, IterableNodeType`。
///
/// 守卫节点：当其条件为假时（或被 negated 时条件为真），触发去优化。
/// 守卫不绑定到特定帧状态，可自由移动，总在调度时使用正确的帧状态。
#[derive(Debug)]
pub struct GuardNode {
    /// 对应 `condition`：产生被测布尔值的逻辑节点。
    pub condition: Box<dyn ValueNode>,
    /// 对应 `reason`：去优化原因。
    pub reason: DeoptimizationReason,
    /// 对应 `action`：去优化动作。
    pub action: DeoptimizationAction,
    /// 对应 `negated`：是否取反——true 表示条件为真时去优化。
    pub negated: bool,
}

impl GuardNode {
    /// 创建一个新的 GuardNode。
    pub fn new(
        condition: Box<dyn ValueNode>,
        reason: DeoptimizationReason,
        action: DeoptimizationAction,
        negated: bool,
    ) -> Self {
        GuardNode {
            condition,
            reason,
            action,
            negated,
        }
    }

    /// 对应 `getCondition()`：获取条件节点。
    pub fn get_condition(&self) -> &dyn ValueNode {
        self.condition.as_ref()
    }

    /// 对应 `setCondition(LogicNode, boolean)`：设置条件节点。
    pub fn set_condition(&mut self, condition: Box<dyn ValueNode>, negated: bool) {
        self.condition = condition;
        self.negated = negated;
    }

    /// 对应 `isNegated()`：是否取反。
    pub fn is_negated(&self) -> bool {
        self.negated
    }

    /// 对应 `getReason()`：去优化原因。
    pub fn get_reason(&self) -> DeoptimizationReason {
        self.reason
    }

    /// 对应 `getAction()`：去优化动作。
    pub fn get_action(&self) -> DeoptimizationAction {
        self.action
    }

    /// 对应 `setAction(DeoptimizationAction)`：设置去优化动作。
    pub fn set_action(&mut self, action: DeoptimizationAction) {
        self.action = action;
    }

    /// 对应 `setReason(DeoptimizationReason)`：设置去优化原因。
    pub fn set_reason(&mut self, reason: DeoptimizationReason) {
        self.reason = reason;
    }

    /// 对应 `negate()`：取反。
    pub fn negate(&mut self) {
        self.negated = !self.negated;
    }

    /// 对应 `deoptsOnTrue()`：是否在条件为真时去优化。
    pub fn deopts_on_true(&self) -> bool {
        self.negated
    }

    /// 对应 `getSpeculation()`：获取推测对象。
    pub fn get_speculation(&self) -> Option<&str> {
        // In the full implementation, this returns a Speculation object from SpeculationLog.
        None
    }

    /// 对应 `setSpeculation(Speculation)`：设置推测对象。
    pub fn set_speculation(&mut self, _speculation: &str) {
        // In the full implementation, this sets a Speculation object.
    }

    /// 对应 `getAnchor()`：获取锚定节点（来自 FloatingAnchoredNode）。
    pub fn get_anchor(&self) -> Option<&dyn crate::nodes::abstract_begin_node::AbstractBeginNode> {
        // In the full implementation, this returns the anchor node.
        None
    }

    /// 对应 `setAnchor(AnchoringNode)`：设置锚定节点。
    pub fn set_anchor(&mut self, _anchor: &dyn crate::nodes::abstract_begin_node::AbstractBeginNode) {
        // In the full implementation, this sets the anchor node.
    }

    /// 对应 `setDeoptimizationReason(DeoptimizationReason, DeoptimizationAction)`：组合设置原因和动作。
    pub fn set_deoptimization_reason(&mut self, reason: DeoptimizationReason, action: DeoptimizationAction) {
        self.reason = reason;
        self.action = action;
    }
}

impl ValueNode for GuardNode {
    fn get_stack_kind(&self) -> rustci_vm_ci::meta::java_kind::JavaKind {
        rustci_vm_ci::meta::java_kind::JavaKind::Void
    }
}

impl GuardingNode for GuardNode {}
