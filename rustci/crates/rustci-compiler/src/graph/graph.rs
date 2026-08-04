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
// Rust mirror of `jdk.graal.compiler.graph.Graph`. Faithful 1:1 port.

use crate::graph::node::{Node, NodeId, Verbosity};
use crate::graph::node_class::NodeClass;
use crate::graph::node_work_list::NodeWorkList;
use crate::graph::typed_graph_node_iterator::TypedGraphNodeIterator;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

/// Unique graph identifier. Each graph gets a unique id for debugging.
pub type GraphId = usize;

/// Corresponds to `public class Graph`.
///
/// The core graph data structure in the Graal compiler IR. A `Graph` holds
/// a collection of `Node`s and manages their creation, deletion, and
/// iteration. Each graph has a unique id and maintains a node-type index
/// for efficient typed iteration over `IterableNodeType` nodes.
///
/// Java class → Rust struct+impl.
#[derive(Debug)]
pub struct Graph {
    /// Corresponds to the `name` field.
    name: String,
    /// Corresponds to the `id` field (unique graph id).
    id: GraphId,
    /// Corresponds to the `nodes` field: the set of live nodes.
    /// Indexed by node id.
    nodes: Vec<Option<Box<dyn Node>>>,
    /// Corresponds to the `nodesDeletedSinceLastCompression` field.
    nodes_deleted_since_last_compression: usize,
    /// Corresponds to the `nodeCount` field (number of live nodes).
    node_count: usize,
    /// Corresponds to the `modCount` field.
    mod_count: u32,
    /// Corresponds to the `nodeIdCount` field (next node id to assign).
    node_id_count: NodeId,
    /// Corresponds to the `iterableNodesLast` arrays (per iterable type).
    /// Maps iterable_id -> list of node ids.
    iterable_nodes: HashMap<u64, Vec<NodeId>>,
    /// Corresponds to the `nodesToIterableId` field.
    nodes_to_iterable_id: HashMap<NodeId, u64>,
    /// Corresponds to the `isAfterExpand` flag.
    is_after_expand: bool,
    /// Corresponds to the `freeIds` set (reusable node ids).
    free_ids: Vec<NodeId>,
    /// Corresponds to the `nodeSourcePosition` field.
    node_source_positions: HashMap<NodeId, crate::graph::node_source_position::NodeSourcePosition>,
    /// Corresponds to the `assumptions` flag.
    has_assumptions: bool,
    /// Corresponds to the `verificationEnabled` flag.
    verification_enabled: bool,
    /// Corresponds to the `debugContext` field (type-erased to avoid circular deps).
    debug_context: Option<Box<dyn Any>>,
}

impl Graph {
    /// Corresponds to `Graph(String name, OptionValues options)`.
    pub fn new(name: String, id: GraphId) -> Self {
        Graph {
            name,
            id,
            nodes: Vec::new(),
            nodes_deleted_since_last_compression: 0,
            node_count: 0,
            mod_count: 0,
            node_id_count: 0,
            iterable_nodes: HashMap::new(),
            nodes_to_iterable_id: HashMap::new(),
            is_after_expand: false,
            free_ids: Vec::new(),
            node_source_positions: HashMap::new(),
            has_assumptions: false,
            verification_enabled: false,
            debug_context: None,
        }
    }

    /// Corresponds to `Graph(String name, OptionValues options)` with options.
    pub fn new_with_options(name: String, id: GraphId, _options: &dyn std::any::Any) -> Self {
        Self::new(name, id)
    }

    /// Corresponds to `String name()`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Corresponds to `int id()` or `long graphId()`.
    pub fn id(&self) -> GraphId {
        self.id
    }

    /// Corresponds to `int getNodeCount()`.
    pub fn get_node_count(&self) -> usize {
        self.node_count
    }

    /// Corresponds to `int getTotalNodeCount()` (includes deleted nodes).
    pub fn get_total_node_count(&self) -> usize {
        self.node_id_count
    }

    /// Corresponds to `int getModCount()`.
    pub fn get_mod_count(&self) -> u32 {
        self.mod_count
    }

    /// Corresponds to `boolean isAfterExpand()`.
    pub fn is_after_expand(&self) -> bool {
        self.is_after_expand
    }

    /// Corresponds to `void setAfterExpand(boolean value)`.
    pub fn set_after_expand(&mut self, value: bool) {
        self.is_after_expand = value;
    }

    /// Corresponds to `boolean hasAssumptions()`.
    pub fn has_assumptions(&self) -> bool {
        self.has_assumptions
    }

    /// Corresponds to `void setHasAssumptions(boolean value)`.
    pub fn set_has_assumptions(&mut self, value: bool) {
        self.has_assumptions = value;
    }

    // ── node management ──

    /// Corresponds to `Node add(Node node)`.
    /// Adds a node to the graph. Returns the node's assigned id.
    pub fn add(&mut self, node: Box<dyn Node>) -> NodeId {
        let id = self.next_node_id();
        self.nodes[id] = Some(node);
        self.node_count += 1;
        self.mod_count = self.mod_count.wrapping_add(1);
        id
    }

    /// Corresponds to `Node addOrGet(Node node)`.
    /// Checks if an equivalent node already exists in the graph.
    /// If found, returns the existing node's id; otherwise adds the node.
    pub fn add_or_get(&mut self, node: Box<dyn Node>) -> NodeId {
        let new_id = node.id();
        // Check if any existing node is value-equal to the new node
        for i in 0..self.nodes.len() {
            if let Some(ref existing) = self.nodes[i] {
                if existing.value_equals(new_id) {
                    // Found an equivalent node; discard the new node and return existing
                    return i;
                }
            }
        }
        // No equivalent node found; add the new one
        self.add(node)
    }

    /// Corresponds to `Node addWithoutUnique(Node node)`.
    /// Adds a node without any uniqueness check.
    pub fn add_without_unique(&mut self, node: Box<dyn Node>) -> NodeId {
        let id = self.next_node_id();
        self.nodes[id] = Some(node);
        self.node_count += 1;
        self.mod_count = self.mod_count.wrapping_add(1);
        id
    }

    /// Corresponds to `void remove(Node node)`.
    pub fn remove(&mut self, node_id: NodeId) {
        if node_id < self.nodes.len() {
            if self.nodes[node_id].is_some() {
                self.nodes[node_id] = None;
                self.node_count -= 1;
                self.nodes_deleted_since_last_compression += 1;
                self.mod_count = self.mod_count.wrapping_add(1);
                self.free_ids.push(node_id);
            }
        }
    }

    /// Corresponds to `Node node(int id)`.
    pub fn node(&self, node_id: NodeId) -> Option<&dyn Node> {
        self.nodes.get(node_id).and_then(|opt| opt.as_ref().map(|b| b.as_ref()))
    }

    /// Corresponds to `Node node(int id)` mutable.
    pub fn node_mut(&mut self, node_id: NodeId) -> Option<&mut dyn Node> {
        self.nodes
            .get_mut(node_id)
            .and_then(|opt| opt.as_mut().map(|b| b.as_mut()))
    }

    /// Corresponds to `int getNodeId(Node node)`.
    /// Returns the node's id if it belongs to this graph, otherwise None.
    pub fn get_node_id(&self, node: &dyn Node) -> Option<NodeId> {
        let id = node.id();
        if id < self.nodes.len() && self.nodes[id].is_some() {
            Some(id)
        } else {
            None
        }
    }

    // ── iteration ──

    /// Corresponds to `Iterable<Node> getNodes()`.
    /// Returns an iterator over all live node ids.
    pub fn get_nodes(&self) -> Vec<NodeId> {
        let mut ids = Vec::with_capacity(self.node_count);
        for (i, node_opt) in self.nodes.iter().enumerate() {
            if node_opt.is_some() {
                ids.push(i);
            }
        }
        ids
    }

    /// Corresponds to `TypedGraphNodeIterator<T> getNodes(Class<T> clazz)`.
    pub fn get_nodes_of_type(&self, iterable_id: u64) -> TypedGraphNodeIterator {
        let node_ids = self
            .iterable_nodes
            .get(&iterable_id)
            .cloned()
            .unwrap_or_default();
        TypedGraphNodeIterator::new(self.id, iterable_id, node_ids)
    }

    /// Corresponds to `int getNodesCount(Class<?> clazz)`.
    pub fn get_nodes_count_of_type(&self, iterable_id: u64) -> usize {
        self.iterable_nodes
            .get(&iterable_id)
            .map(|v| v.len())
            .unwrap_or(0)
    }

    // ── node ordering ──

    /// Corresponds to `Node getNode(int index)`.
    pub fn get_node_at_index(&self, index: usize) -> Option<&dyn Node> {
        self.nodes.get(index).and_then(|opt| opt.as_ref().map(|b| b.as_ref()))
    }

    // ── new node creation ──

    /// Corresponds to `Node addNew(NodeClass<?> nodeClass)`.
    pub fn add_new(&mut self, node_class: &dyn NodeClass) -> NodeId {
        let node = node_class.allocate_instance();
        self.add(node)
    }

    // ── compression ──

    /// Corresponds to `void maybeCompress()`.
    pub fn maybe_compress(&mut self) {
        if self.nodes_deleted_since_last_compression > self.node_count / 2 {
            self.compress();
        }
    }

    /// Corresponds to `void compress()`.
    fn compress(&mut self) {
        let mut old_to_new: HashMap<NodeId, NodeId> = HashMap::new();
        let mut new_id = 0;

        for (old_id, node_opt) in self.nodes.iter().enumerate() {
            if node_opt.is_some() {
                old_to_new.insert(old_id, new_id);
                new_id += 1;
            }
        }

        // Rebuild the nodes array with compressed ids
        let old_nodes = std::mem::take(&mut self.nodes);
        self.nodes = Vec::with_capacity(self.node_count);
        for (_old_id, node_opt) in old_nodes.into_iter().enumerate() {
            if let Some(mut node) = node_opt {
                // Update node's internal edges to use new compressed ids
                for i in 0..node.input_count() {
                    if let Some(old_input) = node.input_at(i) {
                        let new_input = old_to_new.get(&old_input).copied().unwrap_or(old_input);
                        node.initialize_input(i, Some(new_input));
                    }
                }
                for i in 0..node.successor_count() {
                    if let Some(old_succ) = node.successor_at(i) {
                        let new_succ = old_to_new.get(&old_succ).copied().unwrap_or(old_succ);
                        node.initialize_successor(i, Some(new_succ));
                    }
                }
                self.nodes.push(Some(node));
            }
        }

        self.node_id_count = self.node_count;
        self.nodes_deleted_since_last_compression = 0;
        self.free_ids.clear();
        self.mod_count = self.mod_count.wrapping_add(1);
    }

    // ── source positions ──

    /// Corresponds to `NodeSourcePosition getNodeSourcePosition()`.
    pub fn get_node_source_position(
        &self,
        node_id: NodeId,
    ) -> Option<&crate::graph::node_source_position::NodeSourcePosition> {
        self.node_source_positions.get(&node_id)
    }

    /// Corresponds to `void setNodeSourcePosition(NodeSourcePosition pos)`.
    pub fn set_node_source_position(
        &mut self,
        node_id: NodeId,
        pos: crate::graph::node_source_position::NodeSourcePosition,
    ) {
        self.node_source_positions.insert(node_id, pos);
    }

    // ── verification ──

    /// Corresponds to `void verify()`.
    pub fn verify(&self) {
        if !self.verification_enabled {
            return;
        }
        for (i, node_opt) in self.nodes.iter().enumerate() {
            if let Some(node) = node_opt {
                let n: &dyn Node = node.as_ref();
                n.verify();
            }
        }
    }

    /// Corresponds to `void setVerificationEnabled(boolean enabled)`.
    pub fn set_verification_enabled(&mut self, enabled: bool) {
        self.verification_enabled = enabled;
    }

    // ── debug ──

    /// Corresponds to `String toString()`.
    pub fn to_string(&self) -> String {
        format!("Graph({})[{} nodes]", self.name, self.node_count)
    }

    /// Corresponds to `String toString(Verbosity verbosity)`.
    pub fn to_string_with_verbosity(&self, verbosity: Verbosity) -> String {
        match verbosity {
            Verbosity::Id => format!("Graph#{}", self.id),
            Verbosity::Name => format!("Graph({})", self.name),
            _ => {
                let mut s = format!("Graph({})[{} nodes]:\n", self.name, self.node_count);
                for (i, node_opt) in self.nodes.iter().enumerate() {
                    if let Some(node) = node_opt {
                        s.push_str(&format!("  {}: {:?}\n", i, node));
                    }
                }
                s
            }
        }
    }

    // ── debug context ──

    /// Corresponds to `DebugContext getDebugContext()`.
    /// Returns the debug context associated with this graph, if any.
    pub fn get_debug_context(&self) -> Option<&dyn Any> {
        self.debug_context.as_ref().map(|b| b.as_ref())
    }

    /// Corresponds to setting the debug context on the graph.
    pub fn set_debug_context(&mut self, ctx: Box<dyn Any>) {
        self.debug_context = Some(ctx);
    }

    // ── node work list ──

    /// Corresponds to `NodeWorkList getNodeWorkList()`.
    /// Creates a new work list for this graph.
    pub fn get_node_work_list(&self) -> NodeWorkList {
        NodeWorkList::new(self.id, self.node_id_count)
    }

    // ── copy ──

    /// Corresponds to `Graph copy(String name, DebugContext debug)`.
    /// Creates a deep copy of this graph with a new name.
    /// All nodes are cloned using `copy_with_inputs()`.
    pub fn copy(&self, name: String) -> Self {
        let mut new_graph = Graph::new(name, self.id + 1);
        new_graph.debug_context = self.debug_context.as_ref().map(|_| {
            // Type-erased debug context; carry forward if present
            // We cannot clone Box<dyn Any>, so we pass None
            // In practice, the caller should set the debug context
            None
        }).flatten();
        new_graph.has_assumptions = self.has_assumptions;
        new_graph.verification_enabled = self.verification_enabled;
        new_graph.is_after_expand = self.is_after_expand;

        // Clone all nodes
        let mut old_to_new: HashMap<NodeId, NodeId> = HashMap::new();
        for (old_id, node_opt) in self.nodes.iter().enumerate() {
            if let Some(node) = node_opt {
                let cloned = node.copy_with_inputs();
                let new_id = new_graph.add(cloned);
                old_to_new.insert(old_id, new_id);
            }
        }

        // Fix up edges in the cloned graph to use new node ids
        for (old_id, node_opt) in self.nodes.iter().enumerate() {
            if let Some(node) = node_opt {
                if let Some(&new_id) = old_to_new.get(&old_id) {
                    if let Some(cloned) = new_graph.node_mut(new_id) {
                        for i in 0..node.input_count() {
                            if let Some(old_input) = node.input_at(i) {
                                if let Some(&new_input) = old_to_new.get(&old_input) {
                                    cloned.initialize_input(i, Some(new_input));
                                }
                            }
                        }
                        for i in 0..node.successor_count() {
                            if let Some(old_succ) = node.successor_at(i) {
                                if let Some(&new_succ) = old_to_new.get(&old_succ) {
                                    cloned.initialize_successor(i, Some(new_succ));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Copy source positions
        for (old_id, pos) in &self.node_source_positions {
            if let Some(&new_id) = old_to_new.get(old_id) {
                new_graph.node_source_positions.insert(new_id, pos.clone());
            }
        }

        new_graph
    }

    // ── internal helpers ──

    fn next_node_id(&mut self) -> NodeId {
        if let Some(id) = self.free_ids.pop() {
            id
        } else {
            let id = self.node_id_count;
            self.node_id_count += 1;
            if id >= self.nodes.len() {
                self.nodes.resize(id + 1, None);
            }
            id
        }
    }
}