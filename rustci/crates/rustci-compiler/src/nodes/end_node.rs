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

//! 镜像 `jdk.graal.compiler.nodes.EndNode`：合并节点的具体结束节点。
//!
//! 偏离记录：Java `EndNode` 继承 `AbstractEndNode`。Rust 侧为具体 struct。

use rustci_vm_ci::meta::java_kind::JavaKind;

use crate::nodes::abstract_end_node::AbstractEndNode;
use crate::nodes::abstract_merge_node::AbstractMergeNode;
use crate::nodes::fixed_node::FixedNode;
use crate::nodes::value_node::ValueNode;

/// 对应 `final class EndNode extends AbstractEndNode`。
///
/// 表示 MergeNode 的一个输入控制流边。
#[derive(Debug)]
pub struct EndNode {
    pub merge: Option<Box<dyn AbstractMergeNode>>,
}

impl EndNode {
    /// 创建一个新的 EndNode。
    pub fn new() -> Self {
        EndNode { merge: None }
    }
}

impl Default for EndNode {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueNode for EndNode {
    fn get_stack_kind(&self) -> JavaKind {
        JavaKind::Void
    }
}

impl FixedNode for EndNode {}

impl AbstractEndNode for EndNode {
    fn merge(&self) -> Option<&dyn AbstractMergeNode> {
        self.merge
            .as_ref()
            .map(|m| m.as_ref() as &dyn AbstractMergeNode)
    }
}
