/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::collections::HashSet;

/// Represents a collection of InputGraphs.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.GraphContainer`.
pub trait GraphContainer {
    /// Returns the original Group that owns the contained graphs.
    fn get_content_owner(&self) -> &super::group::Group;

    /// Provides event informing about content changes.
    fn get_changed_event(&self) -> &super::changed_event::ChangedEvent<Self>
    where
        Self: Sized;

    /// Returns type ID of the collection.
    fn get_type(&self) -> String;

    /// Decides if the container accepts the graph.
    fn accept(&self, g: &super::input_graph::InputGraph) -> bool;

    /// Name of the container.
    fn get_name(&self) -> String;

    /// Returns number of contained graphs.
    fn get_graphs_count(&self) -> usize;

    /// Returns IDs of all nodes in all contained graphs.
    fn get_child_node_ids(&self) -> HashSet<i32>;

    /// Returns nodes of all child graphs.
    fn get_child_nodes(&self) -> Vec<super::input_node::InputNode>;

    /// Returns list of all graphs.
    fn get_graphs(&self) -> Vec<super::input_graph::InputGraph>;

    /// Returns the last graph in the container.
    fn get_last_graph(&self) -> super::input_graph::InputGraph;

    /// Determines if a node has been changed.
    fn is_node_changed(
        &self,
        base: &super::input_graph::InputGraph,
        to: &super::input_graph::InputGraph,
        node_id: i32,
    ) -> bool;
}