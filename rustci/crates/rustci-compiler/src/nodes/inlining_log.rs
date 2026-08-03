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

//! 镜像 `jdk.graal.compiler.nodes.InliningLog`：内联决策日志。
//!
//! 偏离记录：Java `InliningLog` 包含 `Decision` 和 `Callsite` 内部类，记录所有内联决策。
//! Rust 侧为具体 struct。

use std::collections::HashMap;

/// 对应 `InliningLog.Decision`：单次内联决策。
#[derive(Debug, Clone)]
pub struct InliningDecision {
    /// 对应 `positive`：正向（内联）还是负向（不内联）。
    pub positive: bool,
    /// 对应 `reason`：决策原因。
    pub reason: String,
    /// 对应 `phase`：做出决策的阶段名称。
    pub phase: String,
    /// 对应 `target`：调用目标方法名。
    pub target: String,
}

impl InliningDecision {
    pub fn new(positive: bool, reason: String, phase: String, target: String) -> Self {
        InliningDecision {
            positive,
            reason,
            phase,
            target,
        }
    }

    /// 对应 `isPositive()`。
    pub fn is_positive(&self) -> bool {
        self.positive
    }

    /// 对应 `getReason()`。
    pub fn get_reason(&self) -> &str {
        &self.reason
    }
}

/// 对应 `InliningLog.Callsite`：调用树节点。
#[derive(Debug, Clone)]
pub struct Callsite {
    /// 调用目标方法名。
    pub target: Option<String>,
    /// 内联决策列表。
    pub decisions: Vec<InliningDecision>,
    /// 子调用点。
    pub children: Vec<Callsite>,
    /// 调用点 bci。
    pub bci: i32,
}

impl Callsite {
    pub fn new(target: Option<String>, bci: i32) -> Self {
        Callsite {
            target,
            decisions: Vec::new(),
            children: Vec::new(),
            bci,
        }
    }

    /// 添加子调用点。
    pub fn add_child(&mut self, child: Callsite) {
        self.children.push(child);
    }

    /// 添加内联决策。
    pub fn add_decision(&mut self, decision: InliningDecision) {
        self.decisions.push(decision);
    }
}

/// 对应 `class InliningLog`。
///
/// 包含编译期间对图执行的所有内联决策。
/// 每个内联决策包括：正/负决策、调用目标方法、原因、阶段名称、内联图的日志。
#[derive(Debug, Clone)]
pub struct InliningLog {
    /// 根调用点（对应根编译方法）。
    pub root: Callsite,
    /// 决策总数。
    pub decision_count: usize,
    /// 内联方法名 → 调用点映射。
    pub leaves: HashMap<String, Callsite>,
    /// 叶子调用点（未内联的调用点）。
    pub leaf_callsites: Vec<Callsite>,
}

impl InliningLog {
    /// 创建一个新的 InliningLog。
    pub fn new(root_method: Option<String>) -> Self {
        InliningLog {
            root: Callsite::new(root_method, -1),
            decision_count: 0,
            leaves: HashMap::new(),
            leaf_callsites: Vec::new(),
        }
    }

    /// 对应 `addDecision(boolean, String, String, ResolvedJavaMethod)`：添加内联决策。
    pub fn add_decision(&mut self, positive: bool, reason: String, phase: String, target: String) {
        let decision = InliningDecision::new(positive, reason, phase, target.clone());
        self.root.add_decision(decision);
        self.decision_count += 1;
        if !positive {
            self.leaf_callsites.push(Callsite::new(Some(target), -1));
        }
    }

    /// 对应 `getDecisions()`：获取所有决策。
    pub fn get_decisions(&self) -> &[InliningDecision] {
        &self.root.decisions
    }

    /// 对应 `getDecisionCount()`：获取决策总数。
    pub fn get_decision_count(&self) -> usize {
        self.decision_count
    }

    /// 获取叶子调用点。
    pub fn get_leaf_callsites(&self) -> &[Callsite] {
        &self.leaf_callsites
    }

    /// 对应 `getRootCallsite()`：获取根调用点。
    pub fn get_root_callsite(&self) -> &Callsite {
        &self.root
    }

    /// 对应 `getInlinedMethods()`：获取已内联的方法映射。
    pub fn get_inlined_methods(&self) -> &HashMap<String, Callsite> {
        &self.leaves
    }

    /// 对应 `formatAsTree(boolean)`：格式化为树形结构。
    pub fn format_as_tree(&self, _show_decisions: bool) -> String {
        // In the full implementation, this recursively formats the inlining tree.
        let mut result = String::new();
        result.push_str("Inlining tree:\n");
        if let Some(ref target) = self.root.target {
            result.push_str(&format!("  root: {}\n", target));
        }
        result.push_str(&format!("  decisions: {}\n", self.decision_count));
        result
    }

    /// 对应 `logInliningTree()`：记录内联树到调试日志。
    pub fn log_inlining_tree(&self) {
        // In the full implementation, this logs the tree using DebugContext.
        let _tree = self.format_as_tree(true);
    }

    /// 对应 `getCallsiteFor(ResolvedJavaMethod)`：获取指定方法的调用点。
    pub fn get_callsite_for(&self, method_name: &str) -> Option<&Callsite> {
        self.leaves.get(method_name)
    }

    /// 对应 `addCallsite(Callsite)`：添加调用点。
    pub fn add_callsite(&mut self, callsite: Callsite) {
        if let Some(ref target) = callsite.target {
            self.leaves.insert(target.clone(), callsite.clone());
        }
        self.root.children.push(callsite);
    }

    /// 对应 `addRootCallsite(Callsite)`：添加根调用点。
    pub fn add_root_callsite(&mut self, callsite: Callsite) {
        if let Some(ref target) = callsite.target {
            self.leaves.insert(target.clone(), callsite.clone());
        }
        self.root.children.push(callsite);
    }

    /// 对应 `popInliningDecision()`：弹出内联决策。
    pub fn pop_inlining_decision(&mut self) -> Option<Callsite> {
        self.root.children.pop()
    }

    /// 对应 `getDecisionString(Decision)`：获取决策字符串。
    pub fn get_decision_string(decision: &InliningDecision) -> String {
        let direction = if decision.is_positive() {
            "inline"
        } else {
            "do not inline"
        };
        format!(
            "{}: {} (reason: {}, phase: {})",
            direction, decision.target, decision.reason, decision.phase
        )
    }
}
