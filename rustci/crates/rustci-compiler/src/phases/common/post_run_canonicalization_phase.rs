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

// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
//
// Rust mirror of `jdk.graal.compiler.phases.common.PostRunCanonicalizationPhase`.

use crate::nodes::graph_state::GraphState;
use crate::nodes::structured_graph::StructuredGraph;
use crate::phases::base_phase::{BasePhase, NotApplicable, ALWAYS_APPLICABLE};

/// Post-run canonicalization phase — runs canonicalization after other phases.
/// Mirrors `jdk.graal.compiler.phases.common.PostRunCanonicalizationPhase`.
#[derive(Debug, Clone, Copy)]
pub struct PostRunCanonicalizationPhase;

impl PostRunCanonicalizationPhase {
    /// Singleton instance.
    pub const SINGLETON: Self = Self;
}

impl<C> BasePhase<C> for PostRunCanonicalizationPhase {
    fn not_applicable_to(&self, _graph_state: &GraphState) -> Option<NotApplicable> {
        ALWAYS_APPLICABLE
    }

    fn run(&self, _graph: &mut StructuredGraph, _context: &C) {
        // Post-run canonicalization.
    }

    fn get_name(&self) -> String {
        "PostRunCanonicalizationPhase".to_string()
    }
}