/*
 * Copyright (c) 2013, 2024, Oracle and/or its affiliates. All rights reserved.
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
// Rust mirror of `jdk.graal.compiler.phases.PhaseSuite`.
// A compiler phase that can apply an ordered collection of phases to a graph.

use std::any::TypeId;

use crate::nodes::graph_state::GraphState;
use crate::nodes::structured_graph::StructuredGraph;

use super::base_phase::{BasePhase, NotApplicable, ALWAYS_APPLICABLE};
use super::contract::phase_size_contract::PhaseSizeContract;
use super::placeholder_phase::PlaceholderPhase;
use super::recursive_phase::RecursivePhase;

/// A wrapper that stores a phase together with its concrete TypeId.
/// This allows type-based lookup without requiring `Any` on the trait.
struct PhaseEntry<C: 'static> {
    phase: Box<dyn BasePhase<C>>,
    type_id: TypeId,
}

impl<C: 'static> PhaseEntry<C> {
    fn new(phase: Box<dyn BasePhase<C>>) -> Self {
        let type_id = phase.phase_hash_type_id();
        Self { phase, type_id }
    }
}

/// A compiler phase that can apply an ordered collection of phases to a graph.
/// Mirrors `jdk.graal.compiler.phases.PhaseSuite`.
pub struct PhaseSuite<C: 'static> {
    /// The phases in this suite, in application order.
    entries: Vec<PhaseEntry<C>>,
    /// Whether this suite is immutable.
    immutable: bool,
    /// Records changes made to GraphState by phases. Maps phase index to diff string.
    graph_state_diffs: Option<std::collections::BTreeMap<usize, String>>,
    /// Records the index of the phase that caused failure. -1 means no failure.
    failure_index: i32,
}

impl<C: 'static> PhaseSuite<C> {
    /// Creates a new empty PhaseSuite.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            immutable: false,
            graph_state_diffs: None,
            failure_index: -1,
        }
    }

    /// Returns whether this suite is immutable.
    pub fn is_immutable(&self) -> bool {
        self.immutable
    }

    /// Makes this suite immutable.
    pub fn set_immutable(&mut self) {
        self.immutable = true;
    }

    /// Add a new phase at the beginning of this suite.
    pub fn prepend_phase(&mut self, phase: Box<dyn BasePhase<C>>) {
        assert!(!self.immutable);
        self.entries.insert(0, PhaseEntry::new(phase));
    }

    /// Add a new phase at the end of this suite.
    pub fn append_phase(&mut self, phase: Box<dyn BasePhase<C>>) {
        assert!(!self.immutable);
        self.entries.push(PhaseEntry::new(phase));
    }

    /// Inserts a phase before the last phase in the suite.
    pub fn add_before_last(&mut self, phase: Box<dyn BasePhase<C>>) {
        assert!(!self.immutable);
        let len = self.entries.len();
        if len == 0 {
            self.entries.push(PhaseEntry::new(phase));
        } else {
            self.entries.insert(len - 1, PhaseEntry::new(phase));
        }
    }

    /// Inserts a new phase at the specified index.
    pub fn insert_at_index(&mut self, index: usize, phase: Box<dyn BasePhase<C>>) {
        assert!(!self.immutable);
        self.entries.insert(index, PhaseEntry::new(phase));
    }

    /// Gets an immutable view on the phases in this suite.
    pub fn get_phases(&self) -> Vec<&dyn BasePhase<C>> {
        self.entries.iter().map(|e| e.phase.as_ref()).collect()
    }

    /// Returns the index of the first phase matching the given type, or None.
    pub fn find_phase_index(&self, type_id: TypeId) -> Option<usize> {
        self.entries.iter().position(|e| e.type_id == type_id)
    }

    /// Returns the index of the first phase matching the given type, recursively.
    pub fn find_phase_index_recursive(&self, type_id: TypeId) -> Option<usize> {
        self.entries.iter().position(|e| {
            e.type_id == type_id
                || (e.type_id == TypeId::of::<PlaceholderPhase<C>>()
                    && self.check_placeholder_matches(e, type_id))
        })
    }

    fn check_placeholder_matches(&self, entry: &PhaseEntry<C>, _type_id: TypeId) -> bool {
        entry.type_id == TypeId::of::<PlaceholderPhase<C>>()
    }

    /// Removes the first instance of the given phase type, looking recursively.
    pub fn remove_phase(&mut self, type_id: TypeId) -> bool {
        for i in 0..self.entries.len() {
            if self.entries[i].type_id == type_id {
                self.entries.remove(i);
                return true;
            }
        }
        false
    }

    /// Replaces the first instance of the given phase type.
    pub fn replace_phase(&mut self, type_id: TypeId, new_phase: Box<dyn BasePhase<C>>) -> bool {
        for i in 0..self.entries.len() {
            if self.entries[i].type_id == type_id {
                self.entries[i] = PhaseEntry::new(new_phase);
                return true;
            }
        }
        false
    }

    /// Creates a shallow copy of this suite. Copies all entries.
    pub fn copy(&self) -> Self {
        Self {
            entries: self.entries.iter().map(|e| PhaseEntry {
                phase: e.phase.clone_box(),
                type_id: e.type_id,
            }).collect(),
            immutable: false,
            graph_state_diffs: self.graph_state_diffs.clone(),
            failure_index: self.failure_index,
        }
    }

    /// Returns the graph state diff for the phase at the given position.
    pub fn get_graph_state_diff(&self, position: usize) -> Option<&str> {
        self.graph_state_diffs
            .as_ref()
            .and_then(|diffs| diffs.get(&position))
            .map(|s| s.as_str())
    }

    /// Returns the index of the phase that caused failure.
    pub fn get_failure_index(&self) -> i32 {
        self.failure_index
    }

    /// Returns true if this suite must apply (any of its phases must apply).
    pub fn must_apply(&self, graph_state: &GraphState) -> bool {
        for entry in &self.entries {
            if entry.phase.must_apply(graph_state) {
                return true;
            }
        }
        false
    }

    /// Returns NotApplicable if any phase in this suite cannot be applied.
    pub fn not_applicable_to(&self, graph_state: &GraphState) -> Option<NotApplicable> {
        let mut simulation_graph_state = graph_state.clone_without_log();
        let mut errors = Vec::new();

        for entry in &self.entries {
            let phase_not_applicable = entry.phase.not_applicable_to(&simulation_graph_state);
            if let Some(na) = phase_not_applicable {
                errors.push(format!("{}: {}", entry.phase.get_name(), na));
            }
            entry.phase.update_graph_state(&mut simulation_graph_state);
        }

        if errors.is_empty() {
            ALWAYS_APPLICABLE
        } else {
            Some(NotApplicable::new(errors.join("\n")))
        }
    }

    /// Updates the graph state with all phases in this suite.
    pub fn update_graph_state_with_phases(&self, graph_state: &mut GraphState) {
        for entry in &self.entries {
            entry.phase.update_graph_state(graph_state);
        }
    }
}

impl<C: 'static> PhaseSizeContract for PhaseSuite<C> {
    fn code_size_increase(&self) -> f32 {
        1.0
    }

    fn check_contract(&self) -> bool {
        false
    }

    fn contractor_name(&self) -> String {
        self.get_name()
    }
}

impl<C: 'static> RecursivePhase for PhaseSuite<C> {}

impl<C: 'static> BasePhase<C> for PhaseSuite<C> {
    fn must_apply(&self, graph_state: &GraphState) -> bool {
        for entry in &self.entries {
            if entry.phase.must_apply(graph_state) {
                return true;
            }
        }
        false
    }

    fn not_applicable_to(&self, graph_state: &GraphState) -> Option<NotApplicable> {
        let mut simulation_graph_state = graph_state.clone_without_log();
        let mut errors = Vec::new();

        for entry in &self.entries {
            let phase_not_applicable = entry.phase.not_applicable_to(&simulation_graph_state);
            if let Some(na) = phase_not_applicable {
                errors.push(format!("{}: {}", entry.phase.get_name(), na));
            }
            entry.phase.update_graph_state(&mut simulation_graph_state);
        }

        if errors.is_empty() {
            ALWAYS_APPLICABLE
        } else {
            Some(NotApplicable::new(errors.join("\n")))
        }
    }

    fn run(&self, graph: &mut StructuredGraph, context: &C) {
        for entry in &self.entries {
            entry.phase.apply(graph, context, true);
        }
    }

    fn get_name(&self) -> String {
        "PhaseSuite".to_string()
    }
}

impl<C: 'static> std::fmt::Debug for PhaseSuite<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PhaseSuite")
            .field("phase_count", &self.entries.len())
            .field("immutable", &self.immutable)
            .finish()
    }
}

impl<C: 'static> Default for PhaseSuite<C> {
    fn default() -> Self {
        Self::new()
    }
}