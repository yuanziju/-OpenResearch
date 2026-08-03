/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::collections::HashSet;

/// Classifies graphs into types.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.GraphClassifier`.
pub struct GraphClassifier {
    known_types: HashSet<String>,
}

impl GraphClassifier {
    pub const STRUCTURED_GRAPH: &'static str = "StructuredGraph";
    pub const CALL_GRAPH: &'static str = "CallGraph";
    pub const DEFAULT_TYPE: &'static str = "defaultType";

    /// Creates a new GraphClassifier with the default known types.
    pub fn new() -> Self {
        let mut known_types = HashSet::new();
        known_types.insert(Self::STRUCTURED_GRAPH.to_string());
        known_types.insert(Self::CALL_GRAPH.to_string());
        known_types.insert(Self::DEFAULT_TYPE.to_string());
        GraphClassifier { known_types }
    }

    /// Computes a graph's type from its properties.
    /// Mirrors `GraphClassifier.classifyGraphType(Properties)`.
    pub fn classify_graph_type(&self, properties: &super::properties::Properties) -> String {
        let g = properties.get_string("graph");
        if let Some(g) = g {
            if g.starts_with(Self::STRUCTURED_GRAPH) {
                return Self::STRUCTURED_GRAPH.to_string();
            } else if g.contains("inline") {
                return Self::CALL_GRAPH.to_string();
            }
        }
        Self::DEFAULT_TYPE.to_string()
    }

    /// Returns a set of all the graph types that classifyGraphType can return.
    pub fn known_graph_types(&self) -> &HashSet<String> {
        &self.known_types
    }
}

impl Default for GraphClassifier {
    fn default() -> Self {
        Self::new()
    }
}