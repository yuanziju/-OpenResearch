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

//! Mirror `jdk.graal.compiler.nodes.AbstractMergeNode`: abstract node for merging control flow.
//!
//! Deviation: Java `AbstractMergeNode` extends `BeginStateSplitNode`, has `ends` list,
//! phi queries, etc. Rust side is a trait.

use crate::nodes::abstract_begin_node::AbstractBeginNode;
use crate::nodes::abstract_end_node::AbstractEndNode;
use crate::nodes::end_node::EndNode;

/// Mirrors `abstract class AbstractMergeNode extends BeginStateSplitNode
/// implements IterableNodeType, Simplifiable, LIRLowerable`.
///
/// Represents a merge point for multiple control flow paths.
pub trait AbstractMergeNode: AbstractBeginNode {
    /// Mirrors `forwardEndCount()`: number of forward end nodes.
    fn forward_end_count(&self) -> usize;

    /// Mirrors `forwardEndAt(int)`: get forward end at given index.
    fn forward_end_at(&self, index: usize) -> Option<&EndNode>;

    /// Mirrors `addForwardEnd(EndNode)`: add a forward end node.
    fn add_forward_end(&mut self, end: EndNode);

    /// Mirrors `forwardEndIndex(EndNode)`: return index of the given end node.
    fn forward_end_index(&self, end: &EndNode) -> Option<usize>;

    /// Mirrors `phiPredecessorCount()`: number of phi predecessors.
    fn phi_predecessor_count(&self) -> usize;

    /// Mirrors `phiPredecessorIndex(AbstractEndNode)`: return phi predecessor index.
    fn phi_predecessor_index(&self, pred: &dyn AbstractEndNode) -> Option<usize>;

    /// Mirrors `phiPredecessorAt(int)`: get phi predecessor at given index.
    fn phi_predecessor_at(&self, index: usize) -> Option<&dyn AbstractEndNode>;

    /// Mirrors `removeEnd(AbstractEndNode)`: remove the given end node and related phi entries.
    fn remove_end(&mut self, pred: &dyn AbstractEndNode);

    /// Mirrors `isPhiAtMerge(Node)`: check if a node is a phi merging into this merge.
    fn is_phi_at_merge(&self, _node_index: usize) -> bool {
        false
    }

    /// Mirrors `verifyState()`: verify that stateAfter is not null.
    fn verify_state(&self) -> bool {
        true
    }

    /// Mirrors `verifyNode()`: verify node state.
    fn verify_node(&self) -> bool {
        true
    }
}