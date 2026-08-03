/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::any::Any;

use super::abstract_mutable_document_item::AbstractMutableDocumentItem;
use super::changed_event::ChangedEvent;
use super::changed_event_provider::ChangedEventProvider;
use super::data_collection_event::DataCollectionEvent;
use super::data_collection_listener::DataCollectionListener;
use super::dumped_element::DumpedElement;
use super::folder::Folder;
use super::folder_element::FolderElement;
use super::properties::Properties;

/// The root document of the graph model. Contains groups and properties.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.GraphDocument`.
pub struct GraphDocument {
    base: AbstractMutableDocumentItem,
    elements: Vec<Box<dyn FolderElement + Send + Sync>>,
    properties: Properties,
    changed_event: ChangedEvent<GraphDocument>,
    data_collection_listeners: Vec<Box<dyn DataCollectionListener>>,
}

impl GraphDocument {
    pub fn new(name: String) -> Self {
        let mut doc = GraphDocument {
            base: AbstractMutableDocumentItem::new(name),
            elements: Vec::new(),
            properties: Properties::new(),
            changed_event: ChangedEvent::new_empty(),
            data_collection_listeners: Vec::new(),
        };
        let self_ptr: *const GraphDocument = &doc;
        let self_ref = unsafe { &*self_ptr };
        doc.changed_event.set_object(self_ref);
        doc
    }

    pub fn get_name(&self) -> String {
        self.base.get_name()
    }

    pub fn set_name(&mut self, name: String) {
        self.base.set_name(name);
        self.changed_event.fire();
    }

    pub fn get_properties(&self) -> &Properties {
        &self.properties
    }

    pub fn get_properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    pub fn get_elements(&self) -> &[Box<dyn FolderElement + Send + Sync>] {
        &self.elements
    }

    pub fn add_element(&mut self, element: Box<dyn FolderElement + Send + Sync>) {
        self.elements.push(element);
        self.changed_event.fire();
        let items: Vec<Box<dyn FolderElement + Send + Sync>> = Vec::new();
        let event = DataCollectionEvent::new(self as &(dyn Any + Send + Sync), items, true);
        self.fire_data_loaded(&event);
    }

    pub fn remove_element(&mut self, element: &(dyn FolderElement + Send + Sync)) {
        self.elements.retain(|e| {
            !std::ptr::eq(e.as_ref() as *const _, element as *const _)
        });
        self.changed_event.fire();
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

    pub fn get_elements_count(&self) -> usize {
        self.elements.len()
    }

    pub fn clear(&mut self) {
        let removed = std::mem::take(&mut self.elements);
        let event = DataCollectionEvent::new(self as &(dyn Any + Send + Sync), removed, false);
        self.fire_data_removed(&event);
        self.changed_event.fire();
    }

    fn fire_data_removed(&self, event: &DataCollectionEvent) {
        for listener in &self.data_collection_listeners {
            listener.data_removed(event);
        }
    }
}

impl FolderElement for GraphDocument {
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

impl Folder for GraphDocument {
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
        unimplemented!("GraphDocument::get_changed_event for Box<dyn Folder>")
    }
}

impl ChangedEventProvider<GraphDocument> for GraphDocument {
    fn get_changed_event(&self) -> &ChangedEvent<GraphDocument> {
        &self.changed_event
    }
}

impl DumpedElement for GraphDocument {}

impl std::fmt::Debug for GraphDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphDocument")
            .field("name", &self.base.get_name())
            .field("elements", &self.elements.len())
            .finish()
    }
}

impl PartialEq for GraphDocument {
    fn eq(&self, other: &Self) -> bool {
        self.base.get_name() == other.base.get_name()
    }
}

impl Eq for GraphDocument {}