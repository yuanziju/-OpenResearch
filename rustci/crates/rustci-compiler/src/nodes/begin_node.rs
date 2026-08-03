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

//! 镜像 `jdk.graal.compiler.nodes.BeginNode`：基本块起始的具体节点。

use rustci_vm_ci::meta::java_kind::JavaKind;

use crate::nodes::abstract_begin_node::AbstractBeginNode;
use crate::nodes::fixed_node::FixedNode;
use crate::nodes::fixed_with_next_node::FixedWithNextNode;
use crate::nodes::guarding_node::GuardingNode;
use crate::nodes::value_node::ValueNode;

/// 对应 `final class BeginNode extends AbstractBeginNode implements Simplifiable`。
///
/// 基本块起始节点。可简化——若前驱不是 ControlSplitNode，可被移除。
#[derive(Debug)]
pub struct BeginNode {
    pub next: Option<Box<dyn FixedNode>>,
    pub speculation_fence: bool,
}

impl BeginNode {
    /// 创建一个新的 BeginNode。
    pub fn new() -> Self {
        BeginNode {
            next: None,
            speculation_fence: false,
        }
    }

    /// 对应 `begin(FixedNode)`：在给定节点前创建 BeginNode。
    ///
    /// If `with` is already an AbstractBeginNode, return it directly.
    /// Otherwise, create a new BeginNode, set it as the predecessor of `with`,
    /// and add it to the graph.
    pub fn begin(with: &mut dyn FixedWithNextNode) -> Box<BeginNode> {
        // In the full implementation, this would check if `with` is already
        // an AbstractBeginNode and return it directly. Otherwise, a new
        // BeginNode is created and inserted into the graph before `with`.
        let mut begin = BeginNode::new();
        // The new BeginNode takes the next pointer from `with`,
        // and `with`'s next will be set to the new BeginNode.
        let next_node = with.next();
        // In the full implementation, we would clone the boxed node.
        // For now, we store None since we cannot clone a trait object.
        begin.set_next(None);
        let _ = next_node; // Suppress unused warning
        Box::new(begin)
    }

    /// 对应 `simplify(SimplifierTool)`：
    /// If the predecessor is not a ControlSplitNode, remove this BeginNode
    /// and rewire control flow edges. Guards are moved up to the preceding begin node.
    pub fn simplify(&mut self) -> bool {
        // In the full implementation, this would:
        // 1. Check if predecessor is null (start node) or ControlSplitNode → keep
        // 2. Otherwise, prepareDelete() and remove this BeginNode from the graph
        // 3. Also try PiNode.guardTrySkipPi if predecessor is IfNode
        // Returns true if the node was simplified/removed.
        false
    }

    /// 对应 `prepareDelete()`：疏散锚定值。
    pub fn prepare_delete(&mut self) {
        // In the full implementation, this calls evacuateAnchored(predecessor).
    }

    /// 对应 `prepareDelete(FixedNode)`：带疏散起点的准备删除。
    pub fn prepare_delete_from(&mut self, _evacuate_from: &dyn FixedNode) {
        // In the full implementation, this calls evacuateAnchored(evacuateFrom).
    }

    /// 对应 `verifyNode()`：验证前驱非空或是 graph.start() 或 AbstractMergeNode。
    pub fn verify_node(&self) -> bool {
        // In the full implementation, verifies predecessor is not null or is start/merge.
        true
    }

    /// 对应 `setHasSpeculationFence()`：设置 speculation fence。
    pub fn set_has_speculation_fence(&mut self) {
        self.speculation_fence = true;
    }

    /// 对应 `mustNotMoveAttachedGuards()`：优化器是否允许移动守卫。
    pub fn must_not_move_attached_guards(&self) -> bool {
        false
    }
}

impl Default for BeginNode {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueNode for BeginNode {
    fn get_stack_kind(&self) -> JavaKind {
        JavaKind::Void
    }
}

impl FixedNode for BeginNode {}

impl FixedWithNextNode for BeginNode {
    fn next(&self) -> Option<&dyn FixedNode> {
        self.next.as_ref().map(|n| n.as_ref() as &dyn FixedNode)
    }

    fn set_next(&mut self, next: Option<Box<dyn FixedNode>>) {
        self.next = next;
    }
}

impl GuardingNode for BeginNode {}

impl AbstractBeginNode for BeginNode {
    fn prev_begin(_from: &dyn FixedNode) -> Option<&dyn AbstractBeginNode>
    where
        Self: Sized,
    {
        None
    }

    fn is_used_as_guard_input(&self) -> bool {
        false
    }

    fn has_speculation_fence(&self) -> bool {
        self.speculation_fence
    }
}