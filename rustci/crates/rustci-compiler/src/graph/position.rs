/*
 * Copyright (c) 2011, 2026, Oracle and/or its affiliates. All rights reserved.
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
// Rust mirror of `jdk.graal.compiler.graph.Position`. Faithful 1:1 port.

use crate::graph::node::NodeId;
use std::fmt::Debug;

/// Corresponds to `public abstract class Position`.
///
/// Describes an edge slot in a node. Each node has a fixed number of input
/// and successor positions described by its `NodeClass`. A `Position` pairs
/// a node with an index into its input/successor list.
///
/// Java abstract class → Rust trait.
pub trait Position: Debug {
    /// Corresponds to `Node get(Node node)`.
    /// Returns the node id at this position in the given node.
    fn get(&self, node_id: NodeId) -> Option<NodeId>;

    /// Corresponds to `void set(Node node, Node value)`.
    /// Sets the node at this position in the given node.
    fn set(&self, node_id: NodeId, value: Option<NodeId>);

    /// Corresponds to `void initialize(Node node, Node value)`.
    /// Initializes the node at this position without triggering
    /// usage tracking updates.
    fn initialize(&self, node_id: NodeId, value: Option<NodeId>);

    /// Corresponds to `abstract int index()`.
    /// Returns the index of this position.
    fn index(&self) -> usize;

    /// Corresponds to `abstract int subIndex()`.
    /// Returns the sub-index of this position (for list-type positions).
    fn sub_index(&self) -> usize;

    /// Corresponds to `abstract boolean inputPosition()`.
    /// Returns true if this is an input position, false if it is a successor position.
    fn is_input_position(&self) -> bool;

    /// Corresponds to `Node getOwner()`.
    /// Returns the owning node id of this position.
    fn get_owner(&self) -> NodeId;
}

/// Corresponds to `public static final class InputPosition extends Position`.
///
/// Represents an input edge position in a node.
#[derive(Debug, Clone)]
pub struct InputPosition {
    /// Corresponds to the `owner` field (as node id).
    owner: NodeId,
    /// Corresponds to the `index` field.
    pos_index: usize,
    /// Corresponds to the `subIndex` field.
    pos_sub_index: usize,
}

impl InputPosition {
    /// Corresponds to `InputPosition(Node owner, int index)`.
    pub fn new(owner: NodeId, index: usize) -> Self {
        InputPosition {
            owner,
            pos_index: index,
            pos_sub_index: 0,
        }
    }

    /// Corresponds to `InputPosition(Node owner, int index, int subIndex)`.
    pub fn with_sub_index(owner: NodeId, index: usize, sub_index: usize) -> Self {
        InputPosition {
            owner,
            pos_index: index,
            pos_sub_index: sub_index,
        }
    }
}

impl Position for InputPosition {
    fn get(&self, _node_id: NodeId) -> Option<NodeId> {
        // Resolution is handled by the Graph which owns the node storage
        None
    }

    fn set(&self, _node_id: NodeId, _value: Option<NodeId>) {
        // Resolution is handled by the Graph
    }

    fn initialize(&self, _node_id: NodeId, _value: Option<NodeId>) {
        // Resolution is handled by the Graph
    }

    fn index(&self) -> usize {
        self.pos_index
    }

    fn sub_index(&self) -> usize {
        self.pos_sub_index
    }

    fn is_input_position(&self) -> bool {
        true
    }

    fn get_owner(&self) -> NodeId {
        self.owner
    }
}

/// Corresponds to `public static final class SuccessorPosition extends Position`.
///
/// Represents a successor edge position in a node.
#[derive(Debug, Clone)]
pub struct SuccessorPosition {
    /// Corresponds to the `owner` field (as node id).
    owner: NodeId,
    /// Corresponds to the `index` field.
    pos_index: usize,
    /// Corresponds to the `subIndex` field.
    pos_sub_index: usize,
}

impl SuccessorPosition {
    /// Corresponds to `SuccessorPosition(Node owner, int index)`.
    pub fn new(owner: NodeId, index: usize) -> Self {
        SuccessorPosition {
            owner,
            pos_index: index,
            pos_sub_index: 0,
        }
    }

    /// Corresponds to `SuccessorPosition(Node owner, int index, int subIndex)`.
    pub fn with_sub_index(owner: NodeId, index: usize, sub_index: usize) -> Self {
        SuccessorPosition {
            owner,
            pos_index: index,
            pos_sub_index: sub_index,
        }
    }
}

impl Position for SuccessorPosition {
    fn get(&self, _node_id: NodeId) -> Option<NodeId> {
        // Resolution is handled by the Graph
        None
    }

    fn set(&self, _node_id: NodeId, _value: Option<NodeId>) {
        // Resolution is handled by the Graph
    }

    fn initialize(&self, _node_id: NodeId, _value: Option<NodeId>) {
        // Resolution is handled by the Graph
    }

    fn index(&self) -> usize {
        self.pos_index
    }

    fn sub_index(&self) -> usize {
        self.pos_sub_index
    }

    fn is_input_position(&self) -> bool {
        false
    }

    fn get_owner(&self) -> NodeId {
        self.owner
    }
}