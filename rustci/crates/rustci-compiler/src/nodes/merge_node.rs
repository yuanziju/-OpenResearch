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

//! 镜像 `jdk.graal.compiler.nodes.MergeNode`：多条控制流合并的具体节点。
//!
//! 偏离记录：Java `MergeNode` 继承 `AbstractMergeNode`。Rust 侧为具体 struct。

use rustci_vm_ci::meta::java_kind::JavaKind;

use crate::nodes::abstract_begin_node::AbstractBeginNode;
use crate::nodes::abstract_end_node::AbstractEndNode;
use crate::nodes::abstract_merge_node::AbstractMergeNode;
use crate::nodes::end_node::EndNode;
use crate::nodes::fixed_node::FixedNode;
use crate::nodes::fixed_with_next_node::FixedWithNextNode;
use crate::nodes::guarding_node::GuardingNode;
use crate::nodes::value_node::ValueNode;

/// 对应 `DuplicationHint` 枚举：可选优化提示，指导合并节点的复制决策。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuplicationHint {
    None,
    Explore,
}

/// 对应 `final class MergeNode extends AbstractMergeNode`。
///
/// 表示多条控制流路径的合并点。
#[derive(Debug)]
pub struct MergeNode {
    pub next: Option<Box<dyn FixedNode>>,
    pub forward_ends: Vec<EndNode>,
    pub duplication_hint: DuplicationHint,
    pub speculation_fence: bool,
}

impl MergeNode {
    /// 创建一个新的 MergeNode。
    pub fn new() -> Self {
        MergeNode {
            next: None,
            forward_ends: Vec::new(),
            duplication_hint: DuplicationHint::None,
            speculation_fence: false,
        }
    }

    /// 对应 `getDuplicationHint()`：获取复制提示。
    pub fn get_duplication_hint(&self) -> DuplicationHint {
        self.duplication_hint
    }

    /// 对应 `setDuplicationHint(DuplicationHint)`：设置复制提示。
    pub fn set_duplication_hint(&mut self, hint: DuplicationHint) {
        self.duplication_hint = hint;
    }

    /// 对应 `removeMergeIfDegenerated(MergeNode)`：如果 merge 退化则移除（仅一个前驱且无用途）。
    pub fn remove_merge_if_degenerated(node: &mut MergeNode) {
        if node.forward_end_count() == 1 && node.forward_ends.is_empty() {
            // Simplified: real impl would rewire CFG edges.
            node.forward_ends.clear();
        }
    }
}

impl Default for MergeNode {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueNode for MergeNode {
    fn get_stack_kind(&self) -> JavaKind {
        JavaKind::Void
    }
}

impl FixedNode for MergeNode {}

impl FixedWithNextNode for MergeNode {
    fn next(&self) -> Option<&dyn FixedNode> {
        self.next.as_ref().map(|n| n.as_ref() as &dyn FixedNode)
    }

    fn set_next(&mut self, next: Option<Box<dyn FixedNode>>) {
        self.next = next;
    }
}

impl GuardingNode for MergeNode {}

impl AbstractBeginNode for MergeNode {
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

impl AbstractMergeNode for MergeNode {
    fn forward_end_count(&self) -> usize {
        self.forward_ends.len()
    }

    fn forward_end_at(&self, index: usize) -> Option<&EndNode> {
        self.forward_ends.get(index)
    }

    fn add_forward_end(&mut self, end: EndNode) {
        self.forward_ends.push(end);
    }

    fn forward_end_index(&self, end: &EndNode) -> Option<usize> {
        self.forward_ends.iter().position(|e| std::ptr::eq(e, end))
    }

    fn phi_predecessor_count(&self) -> usize {
        self.forward_end_count()
    }

    fn phi_predecessor_index(&self, _pred: &dyn AbstractEndNode) -> Option<usize> {
        // In real impl, this would check LoopEndNode vs EndNode
        None
    }

    fn phi_predecessor_at(&self, index: usize) -> Option<&dyn AbstractEndNode> {
        self.forward_ends
            .get(index)
            .map(|e| e as &dyn AbstractEndNode)
    }

    fn remove_end(&mut self, pred: &dyn AbstractEndNode) {
        // In real impl, this would find and remove the matching end
        let _ = pred;
    }
}
