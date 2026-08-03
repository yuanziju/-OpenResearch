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

//! 镜像 `jdk.graal.compiler.nodes.LoopBeginNode`：循环起始节点。
//!
//! 偏离记录：Java `LoopBeginNode` 继承 `AbstractMergeNode`。Rust 侧为具体 struct。

use rustci_vm_ci::meta::java_kind::JavaKind;

use crate::nodes::abstract_begin_node::AbstractBeginNode;
use crate::nodes::abstract_end_node::AbstractEndNode;
use crate::nodes::abstract_merge_node::AbstractMergeNode;
use crate::nodes::end_node::EndNode;
use crate::nodes::fixed_node::FixedNode;
use crate::nodes::fixed_with_next_node::FixedWithNextNode;
use crate::nodes::guarding_node::GuardingNode;
use crate::nodes::value_node::ValueNode;

/// 对应 `LoopBeginNode.SafepointState` 枚举：循环 safepoint 状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafepointState {
    MustNeverSafepoint,
    CanSafepoint,
}

impl SafepointState {
    pub fn can_safepoint(self) -> bool {
        self == SafepointState::CanSafepoint
    }
}

/// 对应 `LoopBeginNode.LoopType` 枚举：循环类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopType {
    SimpleLoop,
    PreLoop,
    MainLoop,
    PostLoop,
}

/// 对应 `class LoopBeginNode extends AbstractMergeNode`。
///
/// 表示循环头节点。循环体通过 LoopEndNode 回边，循环出口通过 LoopExitNode。
#[derive(Debug)]
pub struct LoopBeginNode {
    pub next: Option<Box<dyn FixedNode>>,
    pub forward_ends: Vec<EndNode>,
    pub speculation_fence: bool,
    pub loop_ends_safepoint_state: SafepointState,
    pub loop_exits_safepoint_state: SafepointState,
    pub loop_type: LoopType,
    pub unroll_factor: i32,
    pub peelings: i32,
    pub unswitches: i32,
    pub can_never_overflow: bool,
    pub rotated: bool,
    pub osr_loop: bool,
    pub next_end_index: i32,
    /// 可选：循环溢出守卫。
    pub overflow_guard: Option<Box<dyn GuardingNode>>,
}

impl LoopBeginNode {
    /// 创建一个新的 LoopBeginNode。
    pub fn new() -> Self {
        LoopBeginNode {
            next: None,
            forward_ends: Vec::new(),
            speculation_fence: false,
            loop_ends_safepoint_state: SafepointState::CanSafepoint,
            loop_exits_safepoint_state: SafepointState::CanSafepoint,
            loop_type: LoopType::SimpleLoop,
            unroll_factor: 0,
            peelings: 0,
            unswitches: 0,
            can_never_overflow: false,
            rotated: false,
            osr_loop: false,
            next_end_index: 0,
            overflow_guard: None,
        }
    }

    /// 对应 `nextEndIndex()`：分配下一个 LoopEnd 索引。
    pub fn next_end_index(&mut self) -> i32 {
        let idx = self.next_end_index;
        self.next_end_index += 1;
        idx
    }

    /// 对应 `getLoopEndCount()`：获取 LoopEnd 数量。
    pub fn get_loop_end_count(&self) -> i32 {
        self.next_end_index
    }

    /// 对应 `canEndsSafepoint()`：循环尾部是否可 safepoint。
    pub fn can_ends_safepoint(&self) -> bool {
        self.loop_ends_safepoint_state.can_safepoint()
    }

    /// 对应 `canExitsSafepoint()`：循环出口是否可 safepoint。
    pub fn can_exits_safepoint(&self) -> bool {
        self.loop_exits_safepoint_state.can_safepoint()
    }

    /// 对应 `isSimpleLoop()`：是否为简单循环。
    pub fn is_simple_loop(&self) -> bool {
        self.loop_type == LoopType::SimpleLoop
    }

    /// 对应 `setPreLoop()`：标记为预循环。
    pub fn set_pre_loop(&mut self) {
        self.loop_type = LoopType::PreLoop;
    }

    /// 对应 `isPreLoop()`：是否为预循环。
    pub fn is_pre_loop(&self) -> bool {
        self.loop_type == LoopType::PreLoop
    }

    /// 对应 `setMainLoop()`：标记为主循环。
    pub fn set_main_loop(&mut self) {
        self.loop_type = LoopType::MainLoop;
    }

    /// 对应 `isMainLoop()`：是否为主循环。
    pub fn is_main_loop(&self) -> bool {
        self.loop_type == LoopType::MainLoop
    }

    /// 对应 `setPostLoop()`：标记为后循环。
    pub fn set_post_loop(&mut self) {
        self.loop_type = LoopType::PostLoop;
    }

    /// 对应 `isPostLoop()`：是否为后循环。
    pub fn is_post_loop(&self) -> bool {
        self.loop_type == LoopType::PostLoop
    }

    /// 对应 `canOverflow()`：循环是否可能溢出。
    pub fn can_overflow(&self) -> bool {
        !self.can_never_overflow
    }

    /// 对应 `setCanNeverOverflow()`：标记循环永不溢出。
    pub fn set_can_never_overflow(&mut self) {
        self.can_never_overflow = true;
    }

    /// 对应 `isRotated()`：循环是否已旋转。
    pub fn is_rotated(&self) -> bool {
        self.rotated
    }

    /// 对应 `setRotated(boolean)`：设置循环旋转标志。
    pub fn set_rotated(&mut self, rotated: bool) {
        self.rotated = rotated;
    }

    /// 对应 `getUnrollFactor()`：获取展开因子。
    pub fn get_unroll_factor(&self) -> i32 {
        self.unroll_factor
    }

    /// 对应 `setUnrollFactor(int)`：设置展开因子。
    pub fn set_unroll_factor(&mut self, factor: i32) {
        self.unroll_factor = factor;
    }

    /// 对应 `peelings()`：获取剥离次数。
    pub fn peelings(&self) -> i32 {
        self.peelings
    }

    /// 对应 `incrementPeelings()`：增加剥离次数。
    pub fn increment_peelings(&mut self) {
        self.peelings += 1;
    }

    /// 对应 `unswitches()`：获取循环展开次数。
    pub fn unswitches(&self) -> i32 {
        self.unswitches
    }

    /// 对应 `incrementUnswitches()`：增加循环展开次数。
    pub fn increment_unswitches(&mut self) {
        self.unswitches += 1;
    }

    /// 对应 `isOsrLoop()`：是否为 OSR 循环。
    pub fn is_osr_loop(&self) -> bool {
        self.osr_loop
    }

    /// 对应 `markOsrLoop()`：标记为 OSR 循环。
    pub fn mark_osr_loop(&mut self) {
        self.osr_loop = true;
    }

    /// 对应 `getOverflowGuard()`：获取溢出守卫。
    pub fn get_overflow_guard(&self) -> Option<&dyn GuardingNode> {
        self.overflow_guard.as_ref().map(|g| g.as_ref())
    }

    /// 对应 `setOverflowGuard(GuardingNode)`：设置溢出守卫。
    pub fn set_overflow_guard(&mut self, guard: Option<Box<dyn GuardingNode>>) {
        self.overflow_guard = guard;
    }

    /// 对应 `forwardEnd()`：获取前向边（循环入口）。
    pub fn forward_end(&self) -> Option<&EndNode> {
        self.forward_ends.first()
    }

    /// 对应 `loopEnds()`：返回循环尾节点。
    pub fn loop_ends(&self) -> &[EndNode] {
        &self.forward_ends
    }

    /// 对应 `loopExits()`：返回循环出口节点。
    pub fn loop_exits(&self) -> &[EndNode] {
        // In the full implementation, this returns LoopExitNode instances.
        // For now, return empty since we don't track LoopExitNodes separately.
        &[]
    }

    /// 对应 `getLoopEnd(int)`：获取指定索引的 LoopEnd。
    pub fn get_loop_end(&self, index: usize) -> Option<&EndNode> {
        self.forward_ends.get(index)
    }

    /// 对应 `removeLoopEnd(LoopEndNode)`：移除 LoopEnd。
    pub fn remove_loop_end(&mut self, end: &EndNode) {
        self.forward_ends.retain(|e| !std::ptr::eq(e, end));
    }

    /// 对应 `setSafepointState(SafepointState, SafepointState)`：组合设置 safepoint 状态。
    pub fn set_safepoint_state(&mut self, ends: SafepointState, exits: SafepointState) {
        self.loop_ends_safepoint_state = ends;
        self.loop_exits_safepoint_state = exits;
    }

    /// 对应 `disableSafepoint()`：禁用 safepoint。
    pub fn disable_safepoint(&mut self) {
        self.loop_ends_safepoint_state = SafepointState::MustNeverSafepoint;
        self.loop_exits_safepoint_state = SafepointState::MustNeverSafepoint;
    }

    /// 对应 `isCounted()`：是否为计数循环。
    pub fn is_counted(&self) -> bool {
        // In the full implementation, this checks if the loop has a counter-based exit condition.
        // A loop is counted if it has exactly one loop end and can be controlled by a counter.
        false
    }

    /// 对应 `canBeCounted()`：是否可变为计数循环。
    pub fn can_be_counted(&self) -> bool {
        // In the full implementation, this checks if the loop can be converted to a counted loop.
        false
    }
}

impl Default for LoopBeginNode {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueNode for LoopBeginNode {
    fn get_stack_kind(&self) -> JavaKind {
        JavaKind::Void
    }
}

impl FixedNode for LoopBeginNode {}

impl FixedWithNextNode for LoopBeginNode {
    fn next(&self) -> Option<&dyn FixedNode> {
        self.next.as_ref().map(|n| n.as_ref() as &dyn FixedNode)
    }

    fn set_next(&mut self, next: Option<Box<dyn FixedNode>>) {
        self.next = next;
    }
}

impl GuardingNode for LoopBeginNode {}

impl AbstractBeginNode for LoopBeginNode {
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

impl AbstractMergeNode for LoopBeginNode {
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
        self.forward_end_count() + self.get_loop_end_count() as usize
    }

    fn phi_predecessor_index(&self, _pred: &dyn AbstractEndNode) -> Option<usize> {
        None
    }

    fn phi_predecessor_at(&self, index: usize) -> Option<&dyn AbstractEndNode> {
        self.forward_ends
            .get(index)
            .map(|e| e as &dyn AbstractEndNode)
    }

    fn remove_end(&mut self, _pred: &dyn AbstractEndNode) {}
}
