/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use super::super::builder::NodeClass;
use super::known_property_names::KnownPropertyNames;

/// Represents a node in a parsed graph.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.InputNode`.
#[derive(Clone)]
pub struct InputNode {
    id: i32,
    properties: super::properties::Properties,
    class: Option<NodeClass>,
}

impl InputNode {
    pub fn new(id: i32) -> Self {
        InputNode {
            id,
            properties: super::properties::Properties::new(),
            class: None,
        }
    }

    pub fn new_with_properties(id: i32, properties: super::properties::Properties) -> Self {
        InputNode {
            id,
            properties,
            class: None,
        }
    }

    /// Returns the id of this node.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Returns the properties of this node.
    pub fn get_properties(&self) -> &super::properties::Properties {
        &self.properties
    }

    /// Returns the mutable properties of this node.
    pub fn get_properties_mut(&mut self) -> &mut super::properties::Properties {
        &mut self.properties
    }

    /// Returns the node class, if set.
    pub fn get_class(&self) -> Option<&NodeClass> {
        self.class.as_ref()
    }

    /// Sets the node class.
    pub fn set_class(&mut self, class: NodeClass) {
        self.class = Some(class);
    }

    /// Returns the name from properties.
    pub fn get_name(&self) -> String {
        self.properties
            .get_string(KnownPropertyNames::PROPNAME_SHORT_NAME)
            .unwrap_or_else(|| format!("{}", self.id))
    }
}

impl std::fmt::Debug for InputNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InputNode")
            .field("id", &self.id)
            .field("name", &self.get_name())
            .finish()
    }
}

impl PartialEq for InputNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for InputNode {}

impl std::hash::Hash for InputNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}