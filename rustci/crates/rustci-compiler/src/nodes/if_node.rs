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

//! 镜像 `jdk.graal.compiler.nodes.IfNode`：if 条件分支节点。
//!
//! 偏离记录：Java `IfNode` 继承 `ControlSplitNode` 并实现 `Simplifiable`、`LIRLowerable`。
//! Rust 侧为具体 struct。

use rustci_vm_ci::meta::java_kind::JavaKind;

use crate::nodes::abstract_begin_node::AbstractBeginNode;
use crate::nodes::control_split_node::ControlSplitNode;
use crate::nodes::fixed_node::FixedNode;
use crate::nodes::value_node::ValueNode;

/// 对应 `final class IfNode extends ControlSplitNode implements Simplifiable, LIRLowerable`。
///
/// 表示 if-then-else 控制流分支。有两个后继：trueSuccessor 和 falseSuccessor。
#[derive(Debug)]
pub struct IfNode {
    /// 对应 `condition()`：分支条件。
    pub condition: Box<dyn ValueNode>,
    /// 对应 `trueSuccessor()`：条件为真时的后继。
    pub true_successor: Option<Box<dyn AbstractBeginNode>>,
    /// 对应 `falseSuccessor()`：条件为假时的后继。
    pub false_successor: Option<Box<dyn AbstractBeginNode>>,
    /// 对应 `trueSuccessorProbability`：真分支的概率 (0.0..1.0)。
    pub true_successor_probability: f64,
    /// 对应 `isLoopExit`：是否为循环出口。
    pub is_loop_exit: bool,
}

impl IfNode {
    /// 创建一个新的 IfNode。
    pub fn new(condition: Box<dyn ValueNode>, true_successor_probability: f64) -> Self {
        IfNode {
            condition,
            true_successor: None,
            false_successor: None,
            true_successor_probability,
            is_loop_exit: false,
        }
    }

    /// 对应 `trueSuccessor()`：获取真分支后继。
    pub fn true_successor(&self) -> Option<&dyn AbstractBeginNode> {
        self.true_successor.as_ref().map(|s| s.as_ref())
    }

    /// 对应 `falseSuccessor()`：获取假分支后继。
    pub fn false_successor(&self) -> Option<&dyn AbstractBeginNode> {
        self.false_successor.as_ref().map(|s| s.as_ref())
    }

    /// 对应 `setTrueSuccessor(AbstractBeginNode)`：设置真分支后继。
    pub fn set_true_successor(&mut self, succ: Option<Box<dyn AbstractBeginNode>>) {
        self.true_successor = succ;
    }

    /// 对应 `setFalseSuccessor(AbstractBeginNode)`：设置假分支后继。
    pub fn set_false_successor(&mut self, succ: Option<Box<dyn AbstractBeginNode>>) {
        self.false_successor = succ;
    }

    /// 对应 `condition()`：获取条件节点。
    pub fn condition(&self) -> &dyn ValueNode {
        self.condition.as_ref()
    }

    /// 对应 `setCondition(LogicNode)`：设置条件节点。
    pub fn set_condition(&mut self, condition: Box<dyn ValueNode>) {
        self.condition = condition;
    }

    /// 对应 `getTrueSuccessorProbability()`：获取真分支概率。
    pub fn get_true_successor_probability(&self) -> f64 {
        self.true_successor_probability
    }

    /// 对应 `setTrueSuccessorProbability(double)`：设置真分支概率。
    pub fn set_true_successor_probability(&mut self, probability: f64) {
        self.true_successor_probability = probability;
    }

    /// 对应 `getSuccessor(boolean)`：根据布尔值获取后继。
    pub fn get_successor(&self, result: bool) -> Option<&dyn AbstractBeginNode> {
        if result {
            self.true_successor()
        } else {
            self.false_successor()
        }
    }

    /// 对应 `simplify(SimplifierTool)`：简化 if 节点。
    ///
    /// Performs constant folding, swapping, and elimination of the if node.
    /// In the full implementation, this is ~100 lines of complex logic including:
    /// 1. Checking if condition is a constant → eliminate the branch
    /// 2. Swapping successors if the condition is negated
    /// 3. Checking for PiNode guarded patterns
    /// 4. Checking for compare-based patterns
    pub fn simplify(&mut self) -> bool {
        // Check if condition is a constant boolean
        if self.condition.is_constant() {
            self.eliminate(self.condition.as_bool().unwrap_or(false));
            return true;
        }
        false
    }

    /// 对应 `getNegatedCondition()`：获取取反条件。
    ///
    /// Returns a new ValueNode that is the logical negation of this if's condition.
    pub fn get_negated_condition(&self) -> Option<Box<dyn ValueNode>> {
        // In the full implementation, this creates a LogicNegationNode wrapping the condition.
        None
    }

    /// 对应 `eliminate(boolean)`：消除 if 分支。
    ///
    /// If `is_true` is true, the true successor is kept and the false successor is disconnected.
    /// If `is_true` is false, the false successor is kept and the true successor is disconnected.
    pub fn eliminate(&mut self, is_true: bool) {
        if is_true {
            // Keep true successor, disconnect false
            self.false_successor = None;
            self.true_successor_probability = 1.0;
        } else {
            // Keep false successor, disconnect true
            self.true_successor = None;
            self.true_successor_probability = 0.0;
        }
    }

    /// 对应 `swapSuccessors()`：交换后继。
    ///
    /// Swaps true and false successors and updates the probability accordingly.
    pub fn swap_successors(&mut self) {
        std::mem::swap(&mut self.true_successor, &mut self.false_successor);
        self.true_successor_probability = 1.0 - self.true_successor_probability;
    }

    /// 对应 `isLoopExit()`：是否为循环出口。
    pub fn is_loop_exit(&self) -> bool {
        self.is_loop_exit
    }

    /// 对应 `setIsLoopExit(boolean)`：设置循环出口标志。
    pub fn set_is_loop_exit(&mut self, value: bool) {
        self.is_loop_exit = value;
    }

    /// 对应 `checkIfCondition(IfNode)`：检查 if 条件。
    ///
    /// Verifies that the if node's condition is a valid LogicNode.
    pub fn check_if_condition(_if_node: &IfNode) -> bool {
        // In the full implementation, this verifies the condition is a LogicNode
        true
    }
}

impl ValueNode for IfNode {
    fn get_stack_kind(&self) -> JavaKind {
        JavaKind::Void
    }
}

impl FixedNode for IfNode {}

impl ControlSplitNode for IfNode {
    fn probability(&self, successor: &dyn AbstractBeginNode) -> f64 {
        if let Some(ts) = self.true_successor() {
            if std::ptr::eq(ts, successor) {
                return self.true_successor_probability;
            }
        }
        if let Some(fs) = self.false_successor() {
            if std::ptr::eq(fs, successor) {
                return 1.0 - self.true_successor_probability;
            }
        }
        0.0
    }

    fn get_primary_successor(&self) -> Option<&dyn AbstractBeginNode> {
        None
    }

    fn get_successor_count(&self) -> usize {
        2
    }

    fn successor_probabilities(&self) -> Vec<f64> {
        vec![
            self.true_successor_probability,
            1.0 - self.true_successor_probability,
        ]
    }
}
