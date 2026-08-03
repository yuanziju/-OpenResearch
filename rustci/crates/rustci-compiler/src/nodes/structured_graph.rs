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

//! 镜像 `jdk.graal.compiler.nodes.StructuredGraph`：结构化图。
//!
//! 偏离记录：Java `StructuredGraph` 继承 `Graph`，包含 start 节点、调度、内联日志等。
//! Rust 侧为具体 struct，保留核心字段。

use crate::nodes::graph_state::GraphState;
use crate::nodes::inlining_log::InliningLog;
use crate::nodes::start_node::StartNode;

/// 对应 `StructuredGraph.AllowAssumptions` 枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllowAssumptions {
    Yes,
    No,
}

impl AllowAssumptions {
    /// 对应 `ifTrue(boolean)`。
    pub fn if_true(flag: bool) -> Self {
        if flag {
            AllowAssumptions::Yes
        } else {
            AllowAssumptions::No
        }
    }
}

/// 对应 `final class StructuredGraph extends Graph implements JavaMethodContext`。
///
/// 包含至少一个区分节点的图：start 节点。这是控制流的起点。
#[derive(Debug)]
pub struct StructuredGraph {
    /// 对应 `start()`：图的起始节点。
    pub start: StartNode,
    /// 对应 `name`：图名称。
    pub name: Option<String>,
    /// 对应 `graphState`：图状态。
    pub graph_state: GraphState,
    /// 对应 `assumptions`：编译假设。
    pub allow_assumptions: AllowAssumptions,
    /// 对应 `nodeCount`：节点计数。
    pub node_count: u64,
    /// 对应 `inliningLog`：内联日志。
    pub inlining_log: Option<InliningLog>,
    /// 对应 `hasValueProxies`：是否有值代理。
    pub has_value_proxies: bool,
    /// 对应 `frozen`：图是否冻结。
    pub frozen: bool,
}

impl StructuredGraph {
    /// 创建一个新的 StructuredGraph。
    pub fn new(
        name: Option<String>,
        start: StartNode,
        allow_assumptions: AllowAssumptions,
    ) -> Self {
        StructuredGraph {
            start,
            name,
            graph_state: GraphState::new(),
            allow_assumptions,
            node_count: 0,
            inlining_log: None,
            has_value_proxies: false,
            frozen: false,
        }
    }

    /// 对应 `start()`：获取起始节点。
    pub fn start(&self) -> &StartNode {
        &self.start
    }

    /// 对应 `getGraphState()`：获取图状态。
    pub fn get_graph_state(&self) -> &GraphState {
        &self.graph_state
    }

    /// 对应 `getGraphState()` 的可变版本。
    pub fn get_graph_state_mut(&mut self) -> &mut GraphState {
        &mut self.graph_state
    }

    /// 对应 `getNodeCount()`：获取节点计数。
    pub fn get_node_count(&self) -> u64 {
        self.node_count
    }

    /// 增加节点计数。
    pub fn increment_node_count(&mut self) {
        self.node_count += 1;
    }

    /// 对应 `isFrozen()`：图是否冻结（不可修改）。
    pub fn is_frozen(&self) -> bool {
        self.frozen
    }

    /// 对应 `freeze()`：冻结图。
    pub fn freeze(&mut self) {
        self.frozen = true;
    }

    /// 对应 `getInliningLog()`：获取内联日志。
    pub fn get_inlining_log(&self) -> Option<&InliningLog> {
        self.inlining_log.as_ref()
    }

    /// 对应 `setInliningLog(InliningLog)`：设置内联日志。
    pub fn set_inlining_log(&mut self, log: InliningLog) {
        self.inlining_log = Some(log);
    }

    /// 对应 `hasValueProxies()`：是否有值代理。
    pub fn has_value_proxies(&self) -> bool {
        self.has_value_proxies
    }

    /// 对应 `setHasValueProxies(boolean)`：设置值代理标志。
    pub fn set_has_value_proxies(&mut self, value: bool) {
        self.has_value_proxies = value;
    }

    /// 对应 `clearAllStateAfter()`：清除所有 stateAfter。
    pub fn clear_all_state_after(&mut self) {
        // In the full implementation, this iterates all state split nodes
        // and clears their stateAfter references.
    }

    /// 对应 `getNodeCount()`：获取节点计数（已存在）。
    /// 对应 `copy()`：复制图。
    pub fn copy(&self) -> Self {
        // In the full implementation, this deep-copies the graph structure.
        // For now, we clone the struct fields.
        StructuredGraph {
            start: StartNode::new(),
            name: self.name.clone(),
            graph_state: self.graph_state.clone(),
            allow_assumptions: self.allow_assumptions,
            node_count: self.node_count,
            inlining_log: self.inlining_log.clone(),
            has_value_proxies: self.has_value_proxies,
            frozen: false,
        }
    }

    /// 对应 `logInliningTree()`：记录内联树。
    pub fn log_inlining_tree(&self) {
        if let Some(ref log) = self.inlining_log {
            log.log_inlining_tree();
        }
    }

    /// 对应 `maybeCompress()`：可能压缩图。
    pub fn maybe_compress(&mut self) {
        // In the full implementation, this compresses the graph if it's large.
    }
}
