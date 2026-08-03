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

//! 镜像 `jdk.graal.compiler.nodes.GraphState`：图的编译阶段状态。
//!
//! 偏离记录：Java `GraphState` 管理 `StageFlag`、`GuardsStage`、`FrameStateVerification`
//! 等编译进度状态。Rust 侧为具体 struct。

use std::collections::HashSet;

/// 对应 `GraphState.StageFlag` 枚举：编译各阶段标志。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StageFlag {
    PartialEvaluation,
    Canonicalization,
    LoopOverflowsChecked,
    PartialEscape,
    FinalPartialEscape,
    HighTierLowering,
    FloatingReads,
    GuardMovement,
    GuardLowering,
    StripMining,
    ValueProxyRemoval,
    SafepointsInsertion,
    MidTierLowering,
    OptimisticAliasing,
    Fsa,
    NodeVectorization,
    VectorMaterialization,
    OptimisticGuards,
    MidTierBarrierAddition,
    BarrierElimination,
    LowTierLowering,
    VectorLowering,
    ExpandLogic,
    FixedReads,
    LowTierBarrierAddition,
    PartialRedundancySchedule,
    AddressLowering,
    FinalCanonicalization,
    RemoveOpaqueValues,
    TargetVectorLowering,
    FinalSchedule,
}

/// 对应 `GraphState.GuardsStage` 枚举：守卫/去优化阶段。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GuardsStage {
    /// 允许浮动 GuardNode。
    FloatingGuards,
    /// 所有 DeoptimizingNode 必须固定，但可新增。
    FixedDeopts,
    /// FrameState 已关联到 DeoptimizingNode。
    AfterFsa,
}

impl GuardsStage {
    /// 对应 `allowsFloatingGuards()`。
    pub fn allows_floating_guards(self) -> bool {
        self == GuardsStage::FloatingGuards
    }

    /// 对应 `allowsGuardInsertion()`。
    pub fn allows_guard_insertion(self) -> bool {
        self <= GuardsStage::FixedDeopts
    }

    /// 对应 `areFrameStatesAtDeopts()`。
    pub fn are_frame_states_at_deopts(self) -> bool {
        self == GuardsStage::AfterFsa
    }

    /// 对应 `areDeoptsFixed()`。
    pub fn are_deopts_fixed(self) -> bool {
        self >= GuardsStage::FixedDeopts
    }

    /// 对应 `reachedGuardsStage(GuardsStage)`。
    pub fn reached_guards_stage(self, stage: GuardsStage) -> bool {
        self >= stage
    }
}

/// 对应 `GraphState.FrameStateVerification` 枚举：帧状态验证模式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameStateVerification {
    /// 验证所有 AbstractStateSplit（含 LoopExit 和 Merge）。
    All,
    /// 不再验证 LoopExit。
    AllExceptLoopExit,
    /// 不再验证 LoopBegin 和 LoopExit。
    AllExceptLoops,
    /// 验证已禁用。
    None,
}

/// 对应 `GraphState.FrameStateVerificationFeature` 枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrameStateVerificationFeature {
    StateSplits,
    Merges,
    LoopBegins,
    LoopExits,
}

/// 对应 `GraphState.MandatoryStages` 枚举：必需的编译阶段。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MandatoryStages {
    Economy,
    Community,
    Enterprise,
}

/// 对应 `class GraphState`：管理图的编译阶段状态。
#[derive(Debug, Clone)]
pub struct GraphState {
    /// 对应 `stageFlags`：已完成的阶段标志。
    pub stage_flags: HashSet<StageFlag>,
    /// 对应 `guardsStage`：当前守卫阶段。
    pub guards_stage: GuardsStage,
    /// 对应 `frameStateVerification`：帧状态验证模式。
    pub frame_state_verification: FrameStateVerification,
    /// 对应 `futureRequiredStages`：未来需要的阶段。
    pub future_required_stages: HashSet<StageFlag>,
    /// 对应 `disabledFrameStateVerification`：是否禁用了帧状态验证。
    pub disabled_frame_state_verification: bool,
}

impl GraphState {
    /// 创建一个新的 GraphState（默认初始状态）。
    pub fn new() -> Self {
        GraphState {
            stage_flags: HashSet::new(),
            guards_stage: GuardsStage::FloatingGuards,
            frame_state_verification: FrameStateVerification::All,
            future_required_stages: HashSet::new(),
            disabled_frame_state_verification: false,
        }
    }

    /// 对应 `isBeforeStage(StageFlag)`：是否在指定阶段之前。
    pub fn is_before_stage(&self, stage: StageFlag) -> bool {
        !self.stage_flags.contains(&stage)
    }

    /// 对应 `isAfterStage(StageFlag)`：是否已完成指定阶段。
    pub fn is_after_stage(&self, stage: StageFlag) -> bool {
        self.stage_flags.contains(&stage)
    }

    /// 对应 `setAfterStage(StageFlag)`：标记阶段已完成。
    pub fn set_after_stage(&mut self, stage: StageFlag) {
        self.stage_flags.insert(stage);
    }

    /// 对应 `getGuardsStage()`：获取守卫阶段。
    pub fn get_guards_stage(&self) -> GuardsStage {
        self.guards_stage
    }

    /// 对应 `setGuardsStage(GuardsStage)`：设置守卫阶段。
    pub fn set_guards_stage(&mut self, stage: GuardsStage) {
        self.guards_stage = stage;
    }

    /// 对应 `getFrameStateVerification()`：获取帧状态验证模式。
    pub fn get_frame_state_verification(&self) -> FrameStateVerification {
        self.frame_state_verification
    }

    /// 对应 `weakenFrameStateVerification(FrameStateVerification)`：弱化帧状态验证。
    pub fn weaken_frame_state_verification(&mut self, new_verification: FrameStateVerification) {
        self.frame_state_verification = new_verification;
    }

    /// 对应 `forceDisableFrameStateVerification()`：强制禁用帧状态验证。
    pub fn force_disable_frame_state_verification(&mut self) {
        self.frame_state_verification = FrameStateVerification::None;
        self.disabled_frame_state_verification = true;
    }

    /// 对应 `setAfterFSA()`：标记 FSA 已完成。
    pub fn set_after_fsa(&mut self) {
        self.set_guards_stage(GuardsStage::AfterFsa);
        self.set_after_stage(StageFlag::Fsa);
    }

    /// 对应 `requiresFutureStages()`：是否有未来阶段需求。
    pub fn requires_future_stages(&self) -> bool {
        !self.future_required_stages.is_empty()
    }

    /// 对应 `getMandatoryStages()`：获取强制阶段。
    pub fn get_mandatory_stages(&self) -> MandatoryStages {
        // In the full implementation, this is determined by the compiler configuration.
        MandatoryStages::Community
    }

    /// 对应 `checkIfStageIsReachable(StageFlag)`：检查阶段是否可达。
    pub fn check_if_stage_is_reachable(&self, stage: StageFlag) -> bool {
        // A stage is reachable if it hasn't been completed yet
        // or if there are future required stages.
        self.is_before_stage(stage) || self.future_required_stages.contains(&stage)
    }

    /// 对应 `isDuringStage(StageFlag)`：是否正在指定阶段中。
    pub fn is_during_stage(&self, _stage: StageFlag) -> bool {
        // In the full implementation, this checks if the stage is currently being executed.
        // We track completed stages, not currently-executing stages.
        false
    }

    /// 对应 `copy()`：复制图状态。
    pub fn copy(&self) -> Self {
        self.clone()
    }

    /// 对应 `getFutureRequiredStages()`：获取未来需要的阶段。
    pub fn get_future_required_stages(&self) -> &HashSet<StageFlag> {
        &self.future_required_stages
    }

    /// 对应 `addFutureStageRequirement(StageFlag)`：添加未来阶段需求。
    pub fn add_future_stage_requirement(&mut self, stage: StageFlag) {
        self.future_required_stages.insert(stage);
    }
}

impl Default for GraphState {
    fn default() -> Self {
        Self::new()
    }
}
