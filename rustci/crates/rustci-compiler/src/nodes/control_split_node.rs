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

//! Mirror `jdk.graal.compiler.nodes.ControlSplitNode`: abstract base class for control split nodes.
//!
//! Deviation: Java `ControlSplitNode` extends `FixedNode`. Rust side is a trait.

use crate::nodes::abstract_begin_node::AbstractBeginNode;
use crate::nodes::fixed_node::FixedNode;

/// Mirrors `abstract class ControlSplitNode extends FixedNode`.
///
/// Base class for all instructions that split control flow (i.e. have more than one successor).
pub trait ControlSplitNode: FixedNode {
    /// Mirrors `probability(AbstractBeginNode)`: get the probability of the given successor.
    fn probability(&self, successor: &dyn AbstractBeginNode) -> f64;

    /// Mirrors `getPrimarySuccessor()`: return the primary successor.
    /// Data dependencies must be scheduled into the primary successor.
    /// Returns None if no data dependencies are expected.
    fn get_primary_successor(&self) -> Option<&dyn AbstractBeginNode>;

    /// Mirrors `getSuccessorCount()`: return the number of successors.
    fn get_successor_count(&self) -> usize;

    /// Mirrors `successorProbabilities()`: return the probability array for all successors.
    fn successor_probabilities(&self) -> Vec<f64>;

    /// Mirrors `blockSuccessorCount()`: return the number of block successors.
    fn block_successor_count(&self) -> usize {
        self.get_successor_count()
    }

    /// Mirrors `blockSuccessor(int)`: get the block successor at the given index.
    fn block_successor(&self, _index: usize) -> Option<&dyn AbstractBeginNode> {
        None
    }

    /// Mirrors `setBlockSuccessor(int, AbstractBeginNode)`: set the block successor at the given index.
    fn set_block_successor(&mut self, _index: usize, _successor: &dyn AbstractBeginNode) {}

    /// Mirrors `setSuccessorProbabilities(double[])`: set the successor probability array.
    fn set_successor_probabilities(&mut self, _probabilities: &[f64]) {}

    /// Mirrors `getSuccessorProbability(int)`: get the probability of the successor at the given index.
    fn get_successor_probability(&self, _index: usize) -> f64 {
        0.0
    }
}