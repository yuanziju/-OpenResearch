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
// Rust mirror of `jdk.graal.compiler.phases.BasePhase`.
// Base class for all compiler phases. Subclasses should be stateless.
// There is one global instance for each compiler phase shared for all compilations.

use crate::nodes::graph_state::GraphState;
use crate::nodes::graph_state::StageFlag;
use crate::nodes::structured_graph::StructuredGraph;

/// Represents a reason why a phase cannot be applied.
/// Mirrors `jdk.graal.compiler.phases.BasePhase.NotApplicable`.
#[derive(Debug, Clone)]
pub struct NotApplicable {
    /// Describes why a phase cannot be applied at this point of the compilation.
    pub reason: String,
    /// Contains the error that could be thrown if the phase was executed. May be None.
    pub cause: Option<String>,
}

impl NotApplicable {
    /// Creates a new NotApplicable with the given reason.
    pub fn new(reason: String) -> Self {
        Self {
            reason,
            cause: None,
        }
    }

    /// Creates a new NotApplicable with the given reason and cause.
    pub fn with_cause(reason: String, cause: String) -> Self {
        Self {
            reason,
            cause: Some(cause),
        }
    }

    /// Returns ALWAYS_APPLICABLE (None) if `condition` is false, else a NotApplicable.
    pub fn when(condition: bool, reason: &str) -> Option<NotApplicable> {
        if condition {
            Some(NotApplicable::new(reason.to_string()))
        } else {
            None
        }
    }

    /// Returns a NotApplicable with formatted reason if condition is true.
    pub fn when_formatted(condition: bool, reason_format: &str, reason_args: &[&str]) -> Option<NotApplicable> {
        if condition {
            let reason = reason_format.replace("{}", reason_args.first().unwrap_or(&""));
            Some(NotApplicable::new(reason))
        } else {
            None
        }
    }

    /// Returns NotApplicable if graphState is after the given stage flag.
    pub fn unless_run_before(
        phase_name: &str,
        flag: StageFlag,
        graph_state: &GraphState,
    ) -> Option<NotApplicable> {
        Self::when(
            graph_state.is_after_stage(flag),
            &format!("{} must run before the {:?} stage", phase_name, flag),
        )
    }

    /// Returns NotApplicable if graphState is before the given stage flag.
    pub fn unless_run_after(
        phase_name: &str,
        flag: StageFlag,
        graph_state: &GraphState,
    ) -> Option<NotApplicable> {
        Self::when(
            graph_state.is_before_stage(flag),
            &format!(
                "{} must run after the {:?} stage (already applied stages: {:?})",
                phase_name,
                flag,
                graph_state.get_stage_flags()
            ),
        )
    }

    /// Returns NotApplicable if graphState is after the given stage flag.
    /// Equivalent to unless_run_before, but preferred for phases that can be
    /// applied at most once.
    pub fn if_applied(
        phase_name: &str,
        flag: StageFlag,
        graph_state: &GraphState,
    ) -> Option<NotApplicable> {
        Self::when(
            graph_state.is_after_stage(flag),
            &format!(
                "Cannot apply {} because graph is already after {:?} stage",
                phase_name, flag
            ),
        )
    }

    /// Returns NotApplicable if graphState has no speculation log.
    pub fn without_speculation_log(
        phase_name: &str,
        graph_state: &GraphState,
    ) -> Option<NotApplicable> {
        Self::when(
            graph_state.get_speculation_log().is_none(),
            &format!("{} needs a SpeculationLog", phase_name),
        )
    }

    /// Returns the first present element in constraints, or None.
    pub fn if_any(constraints: &[Option<NotApplicable>]) -> Option<NotApplicable> {
        for constraint in constraints {
            if constraint.is_some() {
                return constraint.clone();
            }
        }
        None
    }
}

impl std::fmt::Display for NotApplicable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref cause) = self.cause {
            write!(f, "{}\ncause: {}", self.reason, cause)
        } else {
            write!(f, "{}", self.reason)
        }
    }
}

/// Represents the ALWAYS_APPLICABLE sentinel value.
pub const ALWAYS_APPLICABLE: Option<NotApplicable> = None;

/// Scope for applying a phase. Similar to a DebugCloseable but the close
/// operation gets a Throwable argument indicating whether the run completed
/// normally or with an exception.
/// Mirrors `jdk.graal.compiler.phases.BasePhase.ApplyScope`.
pub trait ApplyScope {
    /// Closes the scope, passing the optional error.
    fn close(&mut self, throwable: Option<&dyn std::error::Error>);
}

/// Options applying to all phases.
/// Mirrors `jdk.graal.compiler.phases.BasePhase.PhaseOptions`.
pub struct PhaseOptions;

impl PhaseOptions {
    /// Verify before-after relation of the relative, computed, code size of a graph.
    pub const VERIFY_GRAAL_PHASES_SIZE_DEFAULT: bool = false;
    /// Minimal size in NodeSize to check the graph size increases of phases.
    pub const MINIMAL_GRAPH_NODE_SIZE_CHECK_SIZE_DEFAULT: i32 = 1000;
}

/// Base trait for all compiler phases.
/// Mirrors `jdk.graal.compiler.phases.BasePhase` and implements `PhaseSizeContract`.
pub trait BasePhase<C> where Self: 'static {
    /// Checks if a phase must be applied at some point in the future for the
    /// compilation of a StructuredGraph to be correct.
    fn must_apply(&self, _graph_state: &GraphState) -> bool {
        false
    }

    /// Returns false if this phase can be skipped for the given graph.
    fn should_apply(&self, _graph: &StructuredGraph) -> bool {
        true
    }

    /// Gets a precondition that prevents applying this phase to a graph whose
    /// state is graph_state. Returns ALWAYS_APPLICABLE (None) by default,
    /// meaning the phase can always be applied.
    fn not_applicable_to(&self, _graph_state: &GraphState) -> Option<NotApplicable> {
        ALWAYS_APPLICABLE
    }

    /// Applies all the changes on the GraphState caused by apply.
    fn update_graph_state(&self, _graph_state: &mut GraphState) {}

    /// Returns whether contracts should be checked for this phase.
    fn check_contract(&self) -> bool {
        true
    }

    /// Returns the name of this phase.
    fn get_name(&self) -> String {
        let type_name = std::any::type_name::<Self>();
        // Extract the simple name from the fully qualified path.
        type_name
            .rsplit("::")
            .next()
            .unwrap_or(type_name)
            .to_string()
    }

    /// Returns the contractor name (same as phase name).
    fn contractor_name(&self) -> String {
        self.get_name()
    }

    /// Returns a factor >= 1 that determines the maximum code size increase.
    fn code_size_increase(&self) -> f32 {
        1.25
    }

    /// Applies this phase to the given graph with the given context.
    fn apply(&self, graph: &mut StructuredGraph, context: &C, dump_graph: bool) {
        if !self.should_apply(graph) {
            self.update_graph_state(&mut graph.graph_state);
            return;
        }

        let cannot_be_applied = self.not_applicable_to(&graph.graph_state);
        if cannot_be_applied.is_some() {
            // In debug mode, this would panic. In release, skip.
            if cfg!(debug_assertions) {
                panic!(
                    "Phase {} cannot be applied: {:?}",
                    self.get_name(),
                    cannot_be_applied
                );
            }
            return;
        }

        // Run the phase.
        let _dump_graph = dump_graph;
        self.run(graph, context);
        self.update_graph_state(&mut graph.graph_state);
    }

    /// Applies this phase to the given graph with the given context (dump enabled).
    fn apply_with_dump(&self, graph: &mut StructuredGraph, context: &C) {
        self.apply(graph, context, true);
    }

    /// Runs this phase. Subclasses must implement this.
    /// For context-free phases (Phase trait), the context is ignored.
    fn run(&self, _graph: &mut StructuredGraph, _context: &C) {
        // Default no-op; override in subclasses.
    }

    /// Returns hash code based on the type name.
    fn phase_hash(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::any::type_name::<Self>().hash(&mut hasher);
        hasher.finish()
    }

    /// Returns the TypeId of the concrete type of this phase.
    /// Used for type-based lookup in PhaseSuite.
    fn phase_hash_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }

    /// Creates a boxed clone of this phase. Default implementation panics;
    /// override for phases that need to be copied (e.g., PhaseSuite).
    fn clone_box(&self) -> Box<dyn BasePhase<C>> {
        panic!("clone_box not implemented for {}", self.get_name());
    }
}