/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::any::Any;

/// Base class for mutable document items in the folder hierarchy.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.AbstractMutableDocumentItem`.
///
/// In Java, this class stores a `ChangedEvent<AbstractMutableDocumentItem>` initialized
/// with `this`. In Rust, we avoid the self-referential struct issue by storing the
/// ChangedEvent in the concrete subclass (Group, InputGraph, etc.) and having
/// the subclass implement `ChangedEventProvider`.
pub struct AbstractMutableDocumentItem {
    name: String,
    parent: Option<Box<dyn super::folder_element::FolderElement + Send + Sync>>,
}

impl AbstractMutableDocumentItem {
    pub fn new(name: String) -> Self {
        AbstractMutableDocumentItem {
            name,
            parent: None,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_parent(&self) -> Option<&(dyn super::folder_element::FolderElement + Send + Sync)> {
        self.parent.as_ref().map(|p| p.as_ref())
    }

    pub fn set_parent(&mut self, parent: Option<Box<dyn super::folder_element::FolderElement + Send + Sync>>) {
        self.parent = parent;
    }
}

impl super::folder_element::FolderElement for AbstractMutableDocumentItem {
    fn get_parent(&self) -> Option<&(dyn super::folder_element::FolderElement + Send + Sync)> {
        self.parent.as_ref().map(|p| p.as_ref())
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_parent(&mut self, parent: Option<Box<dyn super::folder_element::FolderElement + Send + Sync>>) {
        self.parent = parent;
    }

    fn get_id(&self) -> Box<dyn Any> {
        Box::new(self.name.clone())
    }
}

impl super::dumped_element::DumpedElement for AbstractMutableDocumentItem {}

impl Clone for AbstractMutableDocumentItem {
    fn clone(&self) -> Self {
        AbstractMutableDocumentItem {
            name: self.name.clone(),
            parent: None, // parent is not cloned to avoid ownership issues
        }
    }
}