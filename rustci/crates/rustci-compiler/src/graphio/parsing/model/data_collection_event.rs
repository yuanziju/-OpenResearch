/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::any::Any;

/// Event to globally inform that some data is added or removed from the data collection.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.DataCollectionEvent`.
pub struct DataCollectionEvent {
    source_item: *const (),
    new_items: bool,
    items: Vec<Box<dyn super::folder_element::FolderElement + Send + Sync>>,
}

impl DataCollectionEvent {
    /// Creates a new event. `source` is the object that triggered the event.
    pub fn new(
        source: &(dyn Any + Send + Sync),
        items: Vec<Box<dyn super::folder_element::FolderElement + Send + Sync>>,
        new_items: bool,
    ) -> Self {
        DataCollectionEvent {
            source_item: source as *const _ as *const (),
            new_items,
            items,
        }
    }

    pub fn is_new_items(&self) -> bool {
        self.new_items
    }

    pub fn get_items(&self) -> &[Box<dyn super::folder_element::FolderElement + Send + Sync>] {
        &self.items
    }
}