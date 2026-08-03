/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::any::Any;
use std::collections::HashSet;

use super::abstract_mutable_document_item::AbstractMutableDocumentItem;
use super::changed_event::ChangedEvent;
use super::changed_event_provider::ChangedEventProvider;
use super::data_collection_event::DataCollectionEvent;
use super::data_collection_listener::DataCollectionListener;
use super::dumped_element::DumpedElement;
use super::folder::Folder;
use super::folder_element::FolderElement;
use super::graph_container::GraphContainer;
use super::input_graph::InputGraph;
use super::input_method::InputMethod;
use super::input_node::InputNode;
use super::properties::Properties;

/// Represents a group of graphs in the document tree.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.Group`.
pub struct Group {
    base: AbstractMutableDocumentItem,
    elements: Vec<Box<dyn FolderElement + Send + Sync>>,
    graphs: Vec<InputGraph>,
    method: Option<InputMethod>,
    properties: Properties,
    changed_event: ChangedEvent<Group>,
    data_collection_listeners: Vec<Box<dyn DataCollectionListener>>,
}

impl Group {
    pub fn new(name: String) -> Self {
        let mut group = Group {
            base: AbstractMutableDocumentItem::new(name),
            elements: Vec::new(),
            graphs: Vec::new(),
            method: None,
            properties: Properties::new(),
            changed_event: ChangedEvent::new_empty(),
            data_collection_listeners: Vec::new(),
        };
        let self_ptr: *const Group = &group;
        let self_ref = unsafe { &*self_ptr };
        group.changed_event.set_object(self_ref);
        group
    }

    pub fn get_name(&self) -> String {
        self.base.get_name()
    }

    pub fn set_name(&mut self, name: String) {
        self.base.set_name(name);
        self.changed_event.fire();
    }

    pub fn get_method(&self) -> Option<&InputMethod> {
        self.method.as_ref()
    }

    pub fn set_method(&mut self, method: InputMethod) {
        self.method = Some(method);
    }

    pub fn get_properties(&self) -> &Properties {
        &self.properties
    }

    pub fn get_properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    pub fn get_graphs(&self) -> &[InputGraph] {
        &self.graphs
    }

    pub fn add_graph(&mut self, graph: InputGraph) {
        self.graphs.push(graph);
        let items: Vec<Box<dyn FolderElement + Send + Sync>> = Vec::new();
        let event = DataCollectionEvent::new(self as &(dyn Any + Send + Sync), items, true);
        self.fire_data_loaded(&event);
    }

    pub fn add_element(&mut self, element: Box<dyn FolderElement + Send + Sync>) {
        self.elements.push(element);
        self.changed_event.fire();
    }

    pub fn get_elements_count(&self) -> usize {
        self.elements.len()
    }

    pub fn add_data_collection_listener(&mut self, listener: Box<dyn DataCollectionListener>) {
        self.data_collection_listeners.push(listener);
    }

    pub fn remove_data_collection_listener(&mut self, listener: &dyn DataCollectionListener) {
        self.data_collection_listeners.retain(|l| {
            !std::ptr::eq(l.as_ref() as *const _, listener as *const _)
        });
    }

    fn fire_data_loaded(&self, event: &DataCollectionEvent) {
        for listener in &self.data_collection_listeners {
            listener.data_loaded(event);
        }
    }

    fn fire_data_removed(&self, event: &DataCollectionEvent) {
        for listener in &self.data_collection_listeners {
            listener.data_removed(event);
        }
    }
}

impl FolderElement for Group {
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

impl Folder for Group {
    fn get_elements(&self) -> Vec<Box<dyn FolderElement + Send + Sync>> {
        Vec::new()
    }

    fn remove_element(&mut self, element: &(dyn FolderElement + Send + Sync)) {
        self.elements.retain(|e| {
            !std::ptr::eq(e.as_ref() as *const _, element as *const _)
        });
        self.changed_event.fire();
    }

    fn add_element(&mut self, element: Box<dyn FolderElement + Send + Sync>) {
        self.elements.push(element);
        self.changed_event.fire();
    }

    fn get_changed_event(&self) -> &ChangedEvent<Box<dyn Folder + Send + Sync>> {
        unimplemented!("Group::get_changed_event for Box<dyn Folder>")
    }
}

impl ChangedEventProvider<Group> for Group {
    fn get_changed_event(&self) -> &ChangedEvent<Group> {
        &self.changed_event
    }
}

impl GraphContainer for Group {
    fn get_content_owner(&self) -> &Group {
        self
    }

    fn get_changed_event(&self) -> &ChangedEvent<Self> {
        &self.changed_event
    }

    fn get_type(&self) -> String {
        self.properties
            .get_string(super::known_property_names::KnownPropertyNames::PROPNAME_TYPE)
            .unwrap_or_else(|| "defaultType".to_string())
    }

    fn accept(&self, g: &InputGraph) -> bool {
        let g_type = g.get_graph_type();
        let g_type = if g_type.is_empty() { "defaultType" } else { &g_type };
        let my_type = self.get_type();
        g_type == my_type || my_type == "defaultType"
    }

    fn get_name(&self) -> String {
        self.base.get_name()
    }

    fn get_graphs_count(&self) -> usize {
        self.graphs.len()
    }

    fn get_child_node_ids(&self) -> HashSet<i32> {
        let mut ids = HashSet::new();
        for graph in &self.graphs {
            for node in graph.get_nodes() {
                ids.insert(node.get_id());
            }
        }
        ids
    }

    fn get_child_nodes(&self) -> Vec<InputNode> {
        let mut result = Vec::new();
        for graph in &self.graphs {
            result.extend_from_slice(graph.get_nodes());
        }
        result
    }

    fn get_graphs(&self) -> Vec<InputGraph> {
        self.graphs.clone()
    }

    fn get_last_graph(&self) -> InputGraph {
        self.graphs.last().cloned().unwrap_or_else(|| {
            InputGraph::new("empty".to_string(), -1, String::new(), Vec::new())
        })
    }

    fn is_node_changed(
        &self,
        base: &InputGraph,
        to: &InputGraph,
        node_id: i32,
    ) -> bool {
        let base_node = base.get_node(node_id);
        let to_node = to.get_node(node_id);
        match (base_node, to_node) {
            (Some(b), Some(t)) => b != t,
            (None, Some(_)) | (Some(_), None) => true,
            (None, None) => false,
        }
    }
}

impl DumpedElement for Group {}

impl std::fmt::Debug for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Group")
            .field("name", &self.base.get_name())
            .field("graphs", &self.graphs.len())
            .field("elements", &self.elements.len())
            .finish()
    }
}

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        self.base.get_name() == other.base.get_name()
    }
}

impl Eq for Group {}