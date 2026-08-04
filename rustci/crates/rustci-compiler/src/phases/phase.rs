/*
 * Copyright (c) 2011, 2024, Oracle and/or its affiliates. All rights reserved.
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
// Rust mirror of `jdk.graal.compiler.phases.Phase`.

use crate::nodes::structured_graph::StructuredGraph;

use super::base_phase::BasePhase;

/// Base class for compiler phases that don't need a context object.
/// Mirrors `jdk.graal.compiler.phases.Phase`.
/// Rust: Since `Phase` in Java extends `BasePhase<Object>`, this is a trait
/// for phases that don't need a context. Implementors must implement
/// `BasePhase::run()` and ignore the `context: &()` parameter.
pub trait Phase: BasePhase<()> {
    /// Applies this phase to the given graph.
    /// Delegates to `BasePhase::apply()` which calls `run()`.
    fn apply_to_graph(&self, graph: &mut StructuredGraph) {
        self.apply(graph, &(), true);
    }

    /// Applies this phase to the given graph with dump control.
    fn apply_to_graph_with_dump(&self, graph: &mut StructuredGraph, dump_graph: bool) {
        self.apply(graph, &(), dump_graph);
    }
}