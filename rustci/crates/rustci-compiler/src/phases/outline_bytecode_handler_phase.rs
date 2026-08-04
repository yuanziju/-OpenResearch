/*
 * Copyright (c) 2026, Oracle and/or its affiliates. All rights reserved.
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
// Rust mirror of `jdk.graal.compiler.phases.OutlineBytecodeHandlerPhase`.
// This phase is responsible for identifying and processing bytecode handler
// invocations within a given graph.

use crate::nodes::graph_state::GraphState;
use crate::nodes::graph_state::StageFlag;
use crate::nodes::structured_graph::StructuredGraph;

use super::base_phase::{BasePhase, NotApplicable};

/// This phase identifies and processes bytecode handler invocations within a
/// given graph. It provides a framework for outlining these handlers into
/// separate stubs.
/// Mirrors `jdk.graal.compiler.phases.OutlineBytecodeHandlerPhase`.
pub trait OutlineBytecodeHandlerPhase<C>: BasePhase<C> {
    /// Default notApplicableTo: must run before LOOP_OVERFLOWS_CHECKED.
    fn not_applicable_to(&self, graph_state: &GraphState) -> Option<NotApplicable> {
        NotApplicable::unless_run_before(
            &self.get_name(),
            StageFlag::LoopOverflowsChecked,
            graph_state,
        )
    }

    /// Returns whether this phase is applicable to the given enclosing method.
    fn applicable_to(&self, _enclosing_method_name: &str) -> bool {
        true
    }

    /// Called after processing the graph.
    fn after_process_graph(&self, _context: &C, _graph: &mut StructuredGraph) {}

    /// Replaces an invoke node with a call to the outlined handler stub.
    /// Subclasses must implement this.
    fn replace_invoke(
        &self,
        context: &C,
        callsite: &BytecodeHandlerCallSite,
        invoke: &dyn Invoke,
        arguments: &[Box<dyn ValueNode>],
    ) -> Box<dyn FixedNode>;
}

// Placeholder types for the bytecode handler framework.
// These would be fully implemented in the util sub-package.

/// Represents a bytecode handler call site.
/// Mirrors `jdk.graal.compiler.phases.util.BytecodeHandlerCallSite`.
pub struct BytecodeHandlerCallSite {
    pub enclosing_method: String,
    pub bci: i32,
    pub target_method: String,
}

impl BytecodeHandlerCallSite {
    pub fn new(enclosing_method: String, bci: i32, target_method: String) -> Self {
        Self {
            enclosing_method,
            bci,
            target_method,
        }
    }
}

// Forward declarations for types used in the OutlineBytecodeHandlerPhase.
// These are fully defined in the nodes module.

pub trait Invoke {
    fn bci(&self) -> i32;
    fn state_after(&self) -> Option<&dyn FrameState>;
    fn state_during(&self) -> Option<&dyn FrameState>;
    fn as_fixed_node(&self) -> &dyn FixedNode;
    fn call_target(&self) -> &dyn CallTargetNode;
}

pub trait CallTargetNode {
    fn target_method(&self) -> Option<&str>;
    fn arguments(&self) -> &[Box<dyn ValueNode>];
}

pub trait FrameState {
    fn get_method(&self) -> Option<&str>;
}

pub trait ValueNode {
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait FixedNode {
    fn as_any(&self) -> &dyn std::any::Any;
}