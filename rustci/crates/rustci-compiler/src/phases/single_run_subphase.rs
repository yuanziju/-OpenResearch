/*
 * Copyright (c) 2022, Oracle and/or its affiliates. All rights reserved.
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
// Rust mirror of `jdk.graal.compiler.phases.SingleRunSubphase`.

use crate::nodes::structured_graph::StructuredGraph;

use super::base_phase::{ApplyScope, BasePhase};

/// A subclass of BasePhase whose instances may only be applied once.
/// An error will be raised at runtime if a single instance of such a phase
/// is applied more than once.
/// Mirrors `jdk.graal.compiler.phases.SingleRunSubphase`.
pub trait SingleRunSubphase<C>: BasePhase<C> {
    /// Returns whether apply has been called.
    fn apply_called(&self) -> bool;
    /// Sets whether apply has been called.
    fn set_apply_called(&mut self, called: bool);

    /// Override of applyScope to enforce single-run semantics.
    fn apply_scope(&self, _graph: &mut StructuredGraph, _context: &C) -> Option<Box<dyn ApplyScope>> {
        if self.apply_called() {
            panic!("Instances of SingleRunSubphase may only be applied once, but this instance has been applied before.");
        }
        // We cannot set apply_called here because this is &self, not &mut self.
        // The caller must set it.
        None
    }
}

/// A helper struct that wraps a phase and enforces single-run semantics.
/// Unlike the Java version, Rust uses interior mutability for the flag.
pub struct SingleRunGuard<P> {
    phase: P,
    apply_called: std::cell::Cell<bool>,
}

impl<P> SingleRunGuard<P> {
    /// Creates a new single-run guard for the given phase.
    pub fn new(phase: P) -> Self {
        Self {
            phase,
            apply_called: std::cell::Cell::new(false),
        }
    }

    /// Returns a reference to the inner phase.
    pub fn inner(&self) -> &P {
        &self.phase
    }

    /// Returns a mutable reference to the inner phase.
    pub fn inner_mut(&mut self) -> &mut P {
        &mut self.phase
    }

    /// Checks and marks the phase as applied.
    pub fn check_and_mark(&self) {
        if self.apply_called.get() {
            panic!(
                "Instances of SingleRunSubphase may only be applied once, \
                 but this instance has been applied before."
            );
        }
        self.apply_called.set(true);
    }
}