/*
 * Copyright (c) 2011, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::collections::HashMap;

/// Interface that defines the structure of a compiler graph.
///
/// The structure of a graph is composed from nodes with properties, the classes
/// of individual nodes, and ports associated with each node that may contain
/// edges to other nodes. The structure of a graph is assumed to be immutable for
/// the time of GraphOutput operations on it.
pub trait GraphStructure<G, N, C, P> {
    /// Casts `obj` to graph, if possible. Returns `None` if the object does not
    /// represent a graph.
    fn graph(&self, current_graph: &G, obj: &dyn std::any::Any) -> Option<G>;

    /// Returns an iterator over all nodes of the graph.
    fn nodes(&self, graph: &G) -> Vec<N>;

    /// Number of nodes in a graph.
    fn nodes_count(&self, graph: &G) -> usize;

    /// Unique id of a node.
    fn node_id(&self, node: &N) -> i32;

    /// Checks if there is a predecessor for a node.
    fn node_has_predecessor(&self, node: &N) -> bool;

    /// Collects node properties into the provided map.
    fn node_properties(
        &self,
        graph: &G,
        node: &N,
        properties: &mut HashMap<String, Box<dyn std::any::Any>>,
    );

    /// Finds a node for `obj`, if possible.
    fn node(&self, obj: &dyn std::any::Any) -> Option<N>;

    /// Finds a node class for `obj`, if possible.
    fn node_class(&self, obj: &dyn std::any::Any) -> Option<C>;

    /// Finds a node class for `node`.
    fn class_for_node(&self, node: &N) -> C;

    /// The template used to build the name of nodes of this class.
    fn name_template(&self, node_class: &C) -> String;

    /// Java class (or type representation) for a node class.
    fn node_class_type(&self, node_class: &C) -> Box<dyn std::any::Any>;

    /// Input ports of a node class.
    fn port_inputs(&self, node_class: &C) -> P;

    /// Output ports of a node class.
    fn port_outputs(&self, node_class: &C) -> P;

    /// The number of edges in a port.
    fn port_size(&self, port: &P) -> usize;

    /// Checks whether an edge is direct.
    fn edge_direct(&self, port: &P, index: usize) -> bool;

    /// The name of an edge.
    fn edge_name(&self, port: &P, index: usize) -> String;

    /// Type of an edge as an enum object.
    fn edge_type(&self, port: &P, index: usize) -> Box<dyn std::any::Any>;

    /// Nodes where the edges for a port lead to/from.
    fn edge_nodes(&self, graph: &G, node: &N, port: &P, index: usize) -> Option<Vec<N>>;
}
