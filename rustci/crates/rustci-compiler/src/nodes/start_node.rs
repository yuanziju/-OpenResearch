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

//! 镜像 `jdk.graal.compiler.nodes.StartNode`：图的起始节点。
//!
//! 偏离记录：Java `StartNode` 继承 `BeginStateSplitNode` 并实现 `SingleMemoryKill`。
//! Rust 侧为具体 struct。

use rustci_vm_ci::meta::java_kind::JavaKind;

use crate::nodes::abstract_begin_node::AbstractBeginNode;
use crate::nodes::fixed_node::FixedNode;
use crate::nodes::fixed_with_next_node::FixedWithNextNode;
use crate::nodes::guarding_node::GuardingNode;
use crate::nodes::value_node::ValueNode;

/// 对应 `class StartNode extends BeginStateSplitNode implements SingleMemoryKill`。
///
/// 图的起始节点。每个 StructuredGraph 有且仅有一个 StartNode。
#[derive(Debug)]
pub struct StartNode {
    pub next: Option<Box<dyn FixedNode>>,
    pub speculation_fence: bool,
}

impl StartNode {
    /// 创建一个新的 StartNode。
    pub fn new() -> Self {
        StartNode {
            next: None,
            speculation_fence: false,
        }
    }
}

impl Default for StartNode {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueNode for StartNode {
    fn get_stack_kind(&self) -> JavaKind {
        JavaKind::Void
    }
}

impl FixedNode for StartNode {}

impl FixedWithNextNode for StartNode {
    fn next(&self) -> Option<&dyn FixedNode> {
        self.next.as_ref().map(|n| n.as_ref() as &dyn FixedNode)
    }

    fn set_next(&mut self, next: Option<Box<dyn FixedNode>>) {
        self.next = next;
    }
}

impl GuardingNode for StartNode {}

impl AbstractBeginNode for StartNode {
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
