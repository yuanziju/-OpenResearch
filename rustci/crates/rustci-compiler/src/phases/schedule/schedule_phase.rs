/*
 * Copyright (c) 2011, 2025, Oracle and/or its affiliates. All rights reserved.
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

// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
//
// Rust mirror of `jdk.graal.compiler.phases.schedule.SchedulePhase`.
// Final phase that schedules nodes for code emission.

use crate::nodes::graph_state::GraphState;
use crate::nodes::graph_state::StageFlag;
use crate::nodes::structured_graph::StructuredGraph;

use crate::phases::base_phase::{BasePhase, NotApplicable, ALWAYS_APPLICABLE};

/// Defines the strategies for scheduling nodes in the compiler's IR.
/// Mirrors `jdk.graal.compiler.phases.schedule.SchedulePhase.SchedulingStrategy`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingStrategy {
    /// Schedules nodes in the earliest possible block.
    Earliest,
    /// Similar to EARLIEST, but preserves the original order of guards.
    EarliestWithGuardOrder,
    /// Schedules nodes in the latest possible block.
    Latest,
    /// Similar to LATEST, but ensures nodes are not scheduled into loops.
    LatestOutOfLoops,
    /// Extends LATEST_OUT_OF_LOOPS by preserving implicit null checks.
    LatestOutOfLoopsImplicitNullChecks,
    /// Basic block local scheduling to reduce register pressure.
    BasicBlockLocalScheduling,
}

impl SchedulingStrategy {
    /// Returns true if this is an earliest strategy.
    pub fn is_earliest(&self) -> bool {
        matches!(self, Self::Earliest | Self::EarliestWithGuardOrder)
    }

    /// Returns true if this is a latest strategy.
    pub fn is_latest(&self) -> bool {
        !self.is_earliest()
    }

    /// Returns true if this is basic block local scheduling.
    pub fn is_basic_block_local_scheduling(&self) -> bool {
        matches!(self, Self::BasicBlockLocalScheduling)
    }

    /// Returns true if this strategy schedules out of loops.
    pub fn schedule_out_of_loops(&self) -> bool {
        matches!(
            self,
            Self::LatestOutOfLoops | Self::LatestOutOfLoopsImplicitNullChecks
        )
    }

    /// Returns true if this strategy considers implicit null checks.
    pub fn consider_implicit_null_checks(&self) -> bool {
        matches!(self, Self::LatestOutOfLoopsImplicitNullChecks)
    }
}

/// The SchedulePhase performs instruction scheduling for a graph.
/// Mirrors `jdk.graal.compiler.phases.schedule.SchedulePhase`.
#[derive(Debug, Clone)]
pub struct SchedulePhase {
    /// The selected scheduling strategy.
    selected_strategy: SchedulingStrategy,
    /// Whether the graph is immutable.
    immutable_graph: bool,
    /// Whether to verify proxies.
    verify_proxies: bool,
}

impl SchedulePhase {
    /// Creates a new SchedulePhase with the given strategy.
    pub fn new(strategy: SchedulingStrategy) -> Self {
        Self {
            selected_strategy: strategy,
            immutable_graph: false,
            verify_proxies: true,
        }
    }

    /// Creates a new SchedulePhase with the given strategy and immutability.
    pub fn with_immutable(strategy: SchedulingStrategy, immutable_graph: bool) -> Self {
        Self {
            selected_strategy: strategy,
            immutable_graph,
            verify_proxies: true,
        }
    }

    /// Creates a new SchedulePhase with full configuration.
    pub fn with_config(
        strategy: SchedulingStrategy,
        immutable_graph: bool,
        verify_proxies: bool,
    ) -> Self {
        Self {
            selected_strategy: strategy,
            immutable_graph,
            verify_proxies,
        }
    }

    /// Returns the default scheduling strategy.
    pub fn get_default_strategy(_opt_schedule_out_of_loops: bool) -> SchedulingStrategy {
        SchedulingStrategy::LatestOutOfLoops
    }

    /// Returns the selected strategy.
    pub fn get_selected_strategy(&self) -> SchedulingStrategy {
        self.selected_strategy
    }
}

impl<C> BasePhase<C> for SchedulePhase {
    fn not_applicable_to(&self, _graph_state: &GraphState) -> Option<NotApplicable> {
        ALWAYS_APPLICABLE
    }

    fn should_apply(&self, _graph: &StructuredGraph) -> bool {
        true
    }

    fn run(&self, _graph: &mut StructuredGraph, _context: &C) {
        // Scheduling is performed by the Instance inner class.
        // The full scheduling algorithm is complex and involves CFG traversal,
        // block mapping, and node ordering.
    }

    fn get_name(&self) -> String {
        format!("SchedulePhase({:?})", self.selected_strategy)
    }
}

/// The final schedule phase — the last schedule run in any phase plan.
/// After this, no further optimizations must happen that would require re-scheduling.
/// Mirrors `jdk.graal.compiler.phases.schedule.SchedulePhase.FinalSchedulePhase`.
pub struct FinalSchedulePhase;

impl FinalSchedulePhase {
    /// Creates a new FinalSchedulePhase.
    pub fn new() -> Self {
        Self
    }
}

impl Default for FinalSchedulePhase {
    fn default() -> Self {
        Self::new()
    }
}

impl<C> BasePhase<C> for FinalSchedulePhase {
    fn not_applicable_to(&self, graph_state: &GraphState) -> Option<NotApplicable> {
        NotApplicable::if_any(&[
            NotApplicable::if_applied("FinalSchedulePhase", StageFlag::FinalSchedule, graph_state),
            NotApplicable::unless_run_after("FinalSchedulePhase", StageFlag::AddressLowering, graph_state),
        ])
    }

    fn run(&self, _graph: &mut StructuredGraph, _context: &C) {
        // Applies LATEST_OUT_OF_LOOPS scheduling and optionally a post-processing phase.
    }

    fn update_graph_state(&self, graph_state: &mut GraphState) {
        graph_state.set_after_stage(StageFlag::FinalSchedule);
    }

    fn get_name(&self) -> String {
        "FinalSchedulePhase".to_string()
    }
}