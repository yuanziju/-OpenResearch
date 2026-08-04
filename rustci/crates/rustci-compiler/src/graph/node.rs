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
// Rust mirror of `jdk.graal.compiler.graph.Node`. Faithful 1:1 port.

use crate::graph::node_class::NodeClass;
use std::fmt::Debug;

/// Node identifier type. Corresponds to the `id` field in Java `Node`.
/// Each node in a graph has a unique id, which is its index in the node array.
pub type NodeId = usize;

/// A usage record: a node and the position (input index) where this node is used.
#[derive(Debug, Clone, Copy)]
pub struct UsageRecord {
    /// The node that uses another node as input.
    pub node_id: NodeId,
    /// The input index (or successor index) in the using node.
    pub index: usize,
    /// The sub-index for list-type positions.
    pub sub_index: usize,
}

impl UsageRecord {
    /// Corresponds to `create(Node node, int index)`.
    pub fn new(node_id: NodeId, index: usize) -> Self {
        UsageRecord {
            node_id,
            index,
            sub_index: 0,
        }
    }

    /// Corresponds to `create(Node node, int index, int subIndex)`.
    pub fn with_sub_index(node_id: NodeId, index: usize, sub_index: usize) -> Self {
        UsageRecord {
            node_id,
            index,
            sub_index,
        }
    }
}

/// Corresponds to `public abstract class Node implements Cloneable, Formattable`.
///
/// Base class for all nodes in a Graal compiler graph. Defines the core
/// interface for edges (inputs and successors), usage tracking, and
/// graph membership.
///
/// Node references use `NodeId` (usize) rather than trait object references
/// to avoid borrow-checker issues with graph-owned storage.
///
/// Java abstract class → Rust trait.
pub trait Node: Debug {
    // ── identity ──

    /// Corresponds to `int id()`.
    /// Returns the unique id of this node within its graph.
    fn id(&self) -> NodeId;

    /// Corresponds to `NodeClass<? extends Node> getNodeClass()`.
    /// Returns the `NodeClass` metadata for this node's type.
    fn node_class(&self) -> &dyn NodeClass;

    // ── graph membership ──

    /// Corresponds to `Graph graph()`.
    /// Returns the graph id this node belongs to, or None if deleted.
    fn graph_id(&self) -> Option<usize>;

    /// Corresponds to `boolean isAlive()`.
    /// Returns true if this node is currently in a graph.
    fn is_alive(&self) -> bool {
        self.graph_id().is_some()
    }

    /// Corresponds to `boolean isDeleted()`.
    fn is_deleted(&self) -> bool;

    /// Corresponds to `void markDeleted()`.
    fn mark_deleted(&mut self);

    // ── inputs ──

    /// Corresponds to `Node input(int index)`.
    /// Returns the input node id at the given index, or None if null.
    fn input_at(&self, index: usize) -> Option<NodeId>;

    /// Corresponds to `int getInputCount()`.
    fn input_count(&self) -> usize;

    /// Corresponds to `void setInput(int index, Node x)`.
    fn set_input(&mut self, index: usize, value: Option<NodeId>);

    /// Corresponds to `void initializeInput(int index, Node x)`.
    /// Initializes an input without triggering usage tracking.
    fn initialize_input(&mut self, index: usize, value: Option<NodeId>);

    // ── input lists ──

    /// Returns the input at the given list index and sub-index.
    fn input_list_at(&self, _index: usize, _sub_index: usize) -> Option<NodeId> {
        None
    }

    /// Sets the input at the given list index and sub-index.
    fn set_input_list_at(&mut self, _index: usize, _sub_index: usize, _value: Option<NodeId>) {}

    /// Initializes the input at the given list index and sub-index.
    fn initialize_input_list_at(
        &mut self,
        _index: usize,
        _sub_index: usize,
        _value: Option<NodeId>,
    ) {
    }

    // ── successors ──

    /// Corresponds to `Node successor(int index)`.
    /// Returns the successor node id at the given index, or None if null.
    fn successor_at(&self, index: usize) -> Option<NodeId>;

    /// Corresponds to `int getSuccessorCount()`.
    fn successor_count(&self) -> usize;

    /// Corresponds to `void setSuccessor(int index, Node x)`.
    fn set_successor(&mut self, index: usize, value: Option<NodeId>);

    /// Corresponds to `void initializeSuccessor(int index, Node x)`.
    fn initialize_successor(&mut self, index: usize, value: Option<NodeId>);

    // ── successor lists ──

    /// Returns the successor at the given list index and sub-index.
    fn successor_list_at(&self, _index: usize, _sub_index: usize) -> Option<NodeId> {
        None
    }

    /// Sets the successor at the given list index and sub-index.
    fn set_successor_list_at(
        &mut self,
        _index: usize,
        _sub_index: usize,
        _value: Option<NodeId>,
    ) {
    }

    /// Initializes the successor at the given list index and sub-index.
    fn initialize_successor_list_at(
        &mut self,
        _index: usize,
        _sub_index: usize,
        _value: Option<NodeId>,
    ) {
    }

    // ── usages ──

    /// Corresponds to `boolean hasUsages()`.
    /// Returns true if this node is used as an input by any other node.
    fn has_usages(&self) -> bool {
        self.usage_count() > 0
    }

    /// Corresponds to `int getUsageCount()`.
    fn usage_count(&self) -> usize;

    /// Corresponds to `NodeUsageIterator usages()`.
    /// Returns an iterator over all usage records.
    fn usages(&self) -> Vec<UsageRecord>;

    /// Corresponds to `NodeUsageWithCountIterator usagesWithCount()`.
    fn usages_with_count(&self) -> Vec<(UsageRecord, usize)>;

    // ── replacement ──

    /// Corresponds to `void replaceAtUsages(Node other)`.
    /// Replaces this node with `other` at all usages.
    fn replace_at_usages(&mut self, other: NodeId);

    /// Corresponds to `void replaceAtUsages(Node other, Predicate)`.
    fn replace_at_matching_usages(&mut self, other: NodeId, predicate: &dyn Fn(NodeId) -> bool);

    /// Corresponds to `void replaceAtPredecessor(Node other)`.
    fn replace_at_predecessor(&mut self, other: NodeId);

    /// Corresponds to `boolean replaceFirstInput(Node oldInput, Node newInput)`.
    fn replace_first_input(&mut self, old_input: NodeId, new_input: NodeId) -> bool;

    /// Corresponds to `boolean replaceFirstSuccessor(Node oldSuccessor, Node newSuccessor)`.
    fn replace_first_successor(&mut self, old_successor: NodeId, new_successor: NodeId) -> bool;

    // ── deletion ──

    /// Corresponds to `void safeDelete()`.
    /// Safely deletes this node, ensuring it has no usages.
    fn safe_delete(&mut self);

    /// Corresponds to `boolean checkDelete()`.
    fn check_delete(&self) -> bool;

    // ── verification ──

    /// Corresponds to `void verify()`.
    fn verify(&self);

    /// Corresponds to `void verifyInputs()`.
    fn verify_inputs(&self);

    /// Corresponds to `void verifyEdges()`.
    fn verify_edges(&self);

    // ── misc ──

    /// Corresponds to `boolean valueEquals(Node other)`.
    /// Checks if this node is value-equal to another node.
    fn value_equals(&self, other: NodeId) -> bool {
        self.id() == other
    }

    /// Corresponds to `Node copyWithInputs()`.
    /// Creates a copy of this node with the same inputs.
    fn copy_with_inputs(&self) -> Box<dyn Node>
    where
        Self: Sized;

    /// Corresponds to `Map<Object, Object> getDebugProperties()`.
    fn debug_properties(&self) -> std::collections::HashMap<String, String>
    where
        Self: Sized,
    {
        std::collections::HashMap::new()
    }

    /// Corresponds to `long getCreationTime()`.
    fn creation_time(&self) -> u64
    where
        Self: Sized,
    {
        0
    }

    /// Corresponds to `boolean isAllowedUsageType(InputInfo)`.
    fn is_allowed_usage_type(&self, _input_info: &crate::graph::node_class::InputInfo) -> bool {
        true
    }

    /// Corresponds to `void updateUsages(Node oldNode, Node newNode)`.
    fn update_usages(_old_node: NodeId, _new_node: NodeId)
    where
        Self: Sized,
    {
    }

    /// Corresponds to `String toString(Verbosity)`.
    /// Returns a string representation of this node with the given verbosity.
    fn to_string_with_verbosity(&self, verbosity: Verbosity) -> String
    where
        Self: Sized,
    {
        match verbosity {
            Verbosity::Id => format!("{}#{}", std::any::type_name::<Self>(), self.id()),
            Verbosity::Name => format!("{}", std::any::type_name::<Self>()),
            Verbosity::Short => format!("{}@{}", std::any::type_name::<Self>(), self.id()),
            Verbosity::Long => {
                let mut s = format!("{}@{}", std::any::type_name::<Self>(), self.id());
                let props = self.debug_properties();
                if !props.is_empty() {
                    s.push_str(" {");
                    for (k, v) in &props {
                        s.push_str(&format!(" {}={}", k, v));
                    }
                    s.push_str(" }");
                }
                s
            }
            Verbosity::All => format!("{}@{}", std::any::type_name::<Self>(), self.id()),
        }
    }
}

/// Corresponds to the `ModCounts` inner record in Java `Node`.
/// Tracks structural modification counts for the node's edges.
#[derive(Debug, Clone, Default)]
pub struct ModCounts {
    /// Corresponds to `inputsModCount`.
    pub inputs_mod_count: u32,
    /// Corresponds to `successorsModCount`.
    pub successors_mod_count: u32,
}

impl ModCounts {
    /// Corresponds to creating a new `ModCounts`.
    pub fn new() -> Self {
        ModCounts {
            inputs_mod_count: 0,
            successors_mod_count: 0,
        }
    }

    /// Corresponds to `incInputModCount()`.
    pub fn inc_input_mod_count(&mut self) {
        self.inputs_mod_count = self.inputs_mod_count.wrapping_add(1);
    }

    /// Corresponds to `incSuccessorModCount()`.
    pub fn inc_successor_mod_count(&mut self) {
        self.successors_mod_count = self.successors_mod_count.wrapping_add(1);
    }
}

/// Corresponds to the `Verbosity` enum in Java `Node`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    /// Corresponds to `Id`.
    Id,
    /// Corresponds to `Name`.
    Name,
    /// Corresponds to `Short`.
    Short,
    /// Corresponds to `Long`.
    Long,
    /// Corresponds to `All`.
    All,
}