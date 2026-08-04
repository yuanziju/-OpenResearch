/*
 * Copyright (c) 2022, 2023, Oracle and/or its affiliates. All rights reserved.
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
// Rust mirror of `jdk.graal.compiler.phases.PlaceholderPhase`.

use std::any::TypeId;

use crate::nodes::graph_state::GraphState;
use crate::nodes::structured_graph::StructuredGraph;

use super::base_phase::{BasePhase, NotApplicable};

/// Base class to hold a place in a PhaseSuite. This phase should be replaced
/// before execution by an instance of the given phase class.
/// Mirrors `jdk.graal.compiler.phases.PlaceholderPhase`.
pub struct PlaceholderPhase<C: 'static> {
    /// The TypeId of the phase class that this placeholder represents.
    phase_class_type_id: TypeId,
    /// The name of the phase class for debugging.
    phase_class_name: String,
    _phantom: std::marker::PhantomData<C>,
}

impl<C: 'static> PlaceholderPhase<C> {
    /// Creates a new PlaceholderPhase for the given phase class.
    pub fn new(phase_class_type_id: TypeId, phase_class_name: String) -> Self {
        Self {
            phase_class_type_id,
            phase_class_name,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Returns the TypeId of the phase class that should replace this placeholder.
    pub fn get_phase_class_type_id(&self) -> TypeId {
        self.phase_class_type_id
    }

    /// Returns the name of the phase class.
    pub fn get_phase_class_name(&self) -> &str {
        &self.phase_class_name
    }
}

impl<C: 'static> BasePhase<C> for PlaceholderPhase<C> {
    fn not_applicable_to(&self, _graph_state: &GraphState) -> Option<NotApplicable> {
        Some(NotApplicable::new(format!(
            "This is a {} for {}",
            self.get_name(),
            self.phase_class_name
        )))
    }

    fn run(&self, _graph: &mut StructuredGraph, _context: &C) {
        panic!(
            "{} for {} should have been replaced in the phase plan before execution.",
            self.get_name(),
            self.phase_class_name
        );
    }

    fn get_name(&self) -> String {
        format!("PlaceholderPhase({})", self.phase_class_name)
    }

    fn code_size_increase(&self) -> f32 {
        1.0
    }

    fn contractor_name(&self) -> String {
        self.get_name()
    }
}