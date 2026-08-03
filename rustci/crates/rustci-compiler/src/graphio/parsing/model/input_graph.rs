/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::any::Any;
use std::collections::HashMap;

use super::abstract_mutable_document_item::AbstractMutableDocumentItem;
use super::changed_event::ChangedEvent;
use super::changed_event_provider::ChangedEventProvider;
use super::dumped_element::DumpedElement;
use super::folder_element::FolderElement;
use super::input_block::InputBlock;
use super::input_block_edge::InputBlockEdge;
use super::input_edge::InputEdge;
use super::input_node::InputNode;
use super::known_property_names::KnownPropertyNames;
use super::properties::Properties;

/// Represents a parsed graph.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.InputGraph`.
pub struct InputGraph {
    base: AbstractMutableDocumentItem,
    dump_id: i32,
    format: String,
    args: Vec<Box<dyn Any + Send + Sync>>,
    nodes: Vec<InputNode>,
    node_to_index: HashMap<i32, usize>,
    edges: Vec<InputEdge>,
    blocks: Vec<InputBlock>,
    block_edges: Vec<InputBlockEdge>,
    properties: Properties,
    is_duplicate: bool,
    is_dirty: bool,
    changed_event: ChangedEvent<InputGraph>,
}

impl InputGraph {
    pub fn new(name: String, dump_id: i32, format: String, args: Vec<Box<dyn Any + Send + Sync>>) -> Self {
        let mut graph = InputGraph {
            base: AbstractMutableDocumentItem::new(name),
            dump_id,
            format,
            args,
            nodes: Vec::new(),
            node_to_index: HashMap::new(),
            edges: Vec::new(),
            blocks: Vec::new(),
            block_edges: Vec::new(),
            properties: Properties::new(),
            is_duplicate: false,
            is_dirty: false,
            changed_event: ChangedEvent::new_empty(),
        };
        // Set the self-referential pointer. SAFETY: the raw pointer is created
        // from the graph's address and will remain valid for the graph's lifetime.
        let self_ptr: *const InputGraph = &graph;
        let self_ref = unsafe { &*self_ptr };
        graph.changed_event.set_object(self_ref);
        graph
    }

    /// Returns the dump id of this graph.
    pub fn get_dump_id(&self) -> i32 {
        self.dump_id
    }

    /// Returns the format string.
    pub fn get_format(&self) -> &str {
        &self.format
    }

    /// Returns the format arguments.
    pub fn get_args(&self) -> &[Box<dyn Any + Send + Sync>] {
        &self.args
    }

    /// Returns all nodes in the graph.
    pub fn get_nodes(&self) -> &[InputNode] {
        &self.nodes
    }

    /// Returns the number of nodes.
    pub fn get_nodes_count(&self) -> usize {
        self.nodes.len()
    }

    /// Adds a node to the graph.
    pub fn add_node(&mut self, node: InputNode) {
        let id = node.get_id();
        self.node_to_index.insert(id, self.nodes.len());
        self.nodes.push(node);
        self.is_dirty = true;
    }

    /// Returns a node by its id.
    pub fn get_node(&self, id: i32) -> Option<&InputNode> {
        self.node_to_index.get(&id).map(|&idx| &self.nodes[idx])
    }

    /// Returns all edges in the graph.
    pub fn get_edges(&self) -> &[InputEdge] {
        &self.edges
    }

    /// Adds an edge to the graph.
    pub fn add_edge(&mut self, edge: InputEdge) {
        self.edges.push(edge);
        self.is_dirty = true;
    }

    /// Returns all blocks in the graph.
    pub fn get_blocks(&self) -> &[InputBlock] {
        &self.blocks
    }

    /// Adds a block to the graph.
    pub fn add_block(&mut self, block: InputBlock) {
        self.blocks.push(block);
        self.is_dirty = true;
    }

    /// Returns all block edges.
    pub fn get_block_edges(&self) -> &[InputBlockEdge] {
        &self.block_edges
    }

    /// Adds a block edge.
    pub fn add_block_edge(&mut self, edge: InputBlockEdge) {
        self.block_edges.push(edge);
        self.is_dirty = true;
    }

    /// Returns the properties of this graph.
    pub fn get_properties(&self) -> &Properties {
        &self.properties
    }

    /// Returns the mutable properties of this graph.
    pub fn get_properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    /// Returns true if this graph is a duplicate.
    pub fn is_duplicate(&self) -> bool {
        self.is_duplicate
    }

    /// Sets the duplicate flag.
    pub fn set_duplicate(&mut self, duplicate: bool) {
        self.is_duplicate = duplicate;
    }

    /// Returns true if the graph is dirty.
    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    /// Sets the dirty flag.
    pub fn set_dirty(&mut self, dirty: bool) {
        self.is_dirty = dirty;
    }

    /// Returns the name of this graph.
    pub fn get_name(&self) -> String {
        self.base.get_name()
    }

    /// Sets the name of this graph.
    pub fn set_name(&mut self, name: String) {
        self.base.set_name(name);
        self.changed_event.fire();
    }

    /// Returns the graph type from properties.
    pub fn get_graph_type(&self) -> String {
        self.properties
            .get_string(KnownPropertyNames::PROPNAME_TYPE)
            .unwrap_or_default()
    }
}

impl FolderElement for InputGraph {
    fn get_parent(&self) -> Option<&(dyn FolderElement + Send + Sync)> {
        self.base.get_parent()
    }

    fn get_name(&self) -> String {
        self.base.get_name()
    }

    fn set_parent(&mut self, parent: Option<Box<dyn FolderElement + Send + Sync>>) {
        self.base.set_parent(parent);
    }

    fn get_id(&self) -> Box<dyn Any> {
        self.base.get_id()
    }
}

impl ChangedEventProvider<InputGraph> for InputGraph {
    fn get_changed_event(&self) -> &ChangedEvent<InputGraph> {
        &self.changed_event
    }
}

impl DumpedElement for InputGraph {}

impl std::fmt::Debug for InputGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InputGraph")
            .field("name", &self.base.get_name())
            .field("dump_id", &self.dump_id)
            .field("nodes", &self.nodes.len())
            .field("edges", &self.edges.len())
            .finish()
    }
}

impl PartialEq for InputGraph {
    fn eq(&self, other: &Self) -> bool {
        self.dump_id == other.dump_id && self.base.get_name() == other.base.get_name()
    }
}

impl Eq for InputGraph {}

impl Clone for InputGraph {
    fn clone(&self) -> Self {
        let mut graph = InputGraph {
            base: self.base.clone(),
            dump_id: self.dump_id,
            format: self.format.clone(),
            args: Vec::new(), // Box<dyn Any + Send + Sync> is not Clone
            nodes: self.nodes.clone(),
            node_to_index: self.node_to_index.clone(),
            edges: self.edges.clone(),
            blocks: self.blocks.clone(),
            block_edges: self.block_edges.clone(),
            properties: self.properties.clone(),
            is_duplicate: self.is_duplicate,
            is_dirty: self.is_dirty,
            changed_event: ChangedEvent::new_empty(),
        };
        let self_ptr: *const InputGraph = &graph;
        let self_ref = unsafe { &*self_ptr };
        graph.changed_event.set_object(self_ref);
        graph
    }
}