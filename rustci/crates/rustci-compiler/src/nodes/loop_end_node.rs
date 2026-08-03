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

//! 镜像 `jdk.graal.compiler.nodes.LoopEndNode`：循环回边节点。
//!
//! 偏离记录：Java `LoopEndNode` 继承 `AbstractEndNode`。Rust 侧为具体 struct。

use rustci_vm_ci::meta::java_kind::JavaKind;

use crate::nodes::abstract_end_node::AbstractEndNode;
use crate::nodes::abstract_merge_node::AbstractMergeNode;
use crate::nodes::fixed_node::FixedNode;
use crate::nodes::loop_begin_node::{LoopBeginNode, SafepointState};
use crate::nodes::value_node::ValueNode;

/// 对应 `final class LoopEndNode extends AbstractEndNode`。
///
/// 表示循环回边。当到达 LoopEnd 时，执行在循环头处继续。
#[derive(Debug)]
pub struct LoopEndNode {
    /// 对应 `loopBegin`：关联的循环头节点。
    pub loop_begin: Option<Box<LoopBeginNode>>,
    /// 对应 `endIndex`：在循环头 loopEnds 列表中的索引。
    pub end_index: i32,
    /// 对应 `safepointState`：safepoint 状态。
    pub safepoint_state: SafepointState,
}

impl LoopEndNode {
    /// 创建一个新的 LoopEndNode。
    pub fn new(loop_begin: LoopBeginNode) -> Self {
        let idx = loop_begin.next_end_index;
        LoopEndNode {
            loop_begin: Some(Box::new(loop_begin)),
            end_index: idx,
            safepoint_state: SafepointState::CanSafepoint,
        }
    }

    /// 对应 `loopBegin()`：获取循环头节点。
    pub fn loop_begin(&self) -> Option<&LoopBeginNode> {
        self.loop_begin.as_ref().map(|lb| lb.as_ref())
    }

    /// 对应 `setLoopBegin(LoopBeginNode)`：设置循环头节点。
    pub fn set_loop_begin(&mut self, loop_begin: LoopBeginNode) {
        self.loop_begin = Some(Box::new(loop_begin));
    }

    /// 对应 `endIndex()`：获取在循环头中的索引。
    pub fn end_index(&self) -> i32 {
        self.end_index
    }

    /// 对应 `setEndIndex(int)`：设置索引。
    pub fn set_end_index(&mut self, idx: i32) {
        self.end_index = idx;
    }

    /// 对应 `getSafepointState()`：获取 safepoint 状态。
    pub fn get_safepoint_state(&self) -> SafepointState {
        self.safepoint_state
    }

    /// 对应 `setSafepointState(SafepointState)`：设置 safepoint 状态。
    pub fn set_safepoint_state(&mut self, state: SafepointState) {
        self.safepoint_state = state;
    }

    /// 对应 `canSafepoint()`：检查 safepoint 状态。
    pub fn can_safepoint(&self) -> bool {
        self.safepoint_state.can_safepoint()
    }
}

impl ValueNode for LoopEndNode {
    fn get_stack_kind(&self) -> JavaKind {
        JavaKind::Void
    }
}

impl FixedNode for LoopEndNode {}

impl AbstractEndNode for LoopEndNode {
    fn merge(&self) -> Option<&dyn AbstractMergeNode> {
        self.loop_begin
            .as_ref()
            .map(|lb| lb.as_ref() as &dyn AbstractMergeNode)
    }
}
