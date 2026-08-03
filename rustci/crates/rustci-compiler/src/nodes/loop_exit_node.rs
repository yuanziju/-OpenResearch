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

//! 镜像 `jdk.graal.compiler.nodes.LoopExitNode`：循环出口节点。
//!
//! 偏离记录：Java `LoopExitNode` 继承 `BeginStateSplitNode` 并实现 `IterableNodeType`、`Simplifiable`。
//! Rust 侧为具体 struct。

use rustci_vm_ci::meta::java_kind::JavaKind;

use crate::nodes::abstract_begin_node::AbstractBeginNode;
use crate::nodes::fixed_node::FixedNode;
use crate::nodes::fixed_with_next_node::FixedWithNextNode;
use crate::nodes::guarding_node::GuardingNode;
use crate::nodes::loop_begin_node::LoopBeginNode;
use crate::nodes::value_node::ValueNode;

/// 对应 `final class LoopExitNode extends BeginStateSplitNode implements IterableNodeType, Simplifiable`。
///
/// 表示循环出口。离开循环的边从此节点开始。
#[derive(Debug)]
pub struct LoopExitNode {
    pub next: Option<Box<dyn FixedNode>>,
    pub speculation_fence: bool,
    /// 对应 `loopBegin`：关联的循环头节点。
    pub loop_begin: Option<Box<LoopBeginNode>>,
}

impl LoopExitNode {
    /// 创建一个新的 LoopExitNode。
    pub fn new(loop_begin: LoopBeginNode) -> Self {
        LoopExitNode {
            next: None,
            speculation_fence: false,
            loop_begin: Some(Box::new(loop_begin)),
        }
    }

    /// 对应 `loopBegin()`：获取循环头节点。
    pub fn loop_begin(&self) -> Option<&LoopBeginNode> {
        self.loop_begin.as_ref().map(|lb| lb.as_ref())
    }

    /// 对应 `setLoopBegin(AbstractBeginNode)`：设置循环头节点。
    pub fn set_loop_begin(&mut self, loop_begin: LoopBeginNode) {
        self.loop_begin = Some(Box::new(loop_begin));
    }
}

impl ValueNode for LoopExitNode {
    fn get_stack_kind(&self) -> JavaKind {
        JavaKind::Void
    }
}

impl FixedNode for LoopExitNode {}

impl FixedWithNextNode for LoopExitNode {
    fn next(&self) -> Option<&dyn FixedNode> {
        self.next.as_ref().map(|n| n.as_ref() as &dyn FixedNode)
    }

    fn set_next(&mut self, next: Option<Box<dyn FixedNode>>) {
        self.next = next;
    }
}

impl GuardingNode for LoopExitNode {}

impl AbstractBeginNode for LoopExitNode {
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
