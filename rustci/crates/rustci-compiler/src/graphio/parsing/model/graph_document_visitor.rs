/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Visitor interface to traverse the various elements of the GraphDocument tree.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.GraphDocumentVisitor`.
pub trait GraphDocumentVisitor {
    /// Visits a Folder.
    fn visit_folder(&mut self, folder: &(dyn super::folder::Folder + Send + Sync)) -> bool {
        for elem in folder.get_elements() {
            if !self.visit_folder_element(elem.as_ref()) {
                return false;
            }
        }
        true
    }

    /// Visits an InputGraph.
    fn visit_input_graph(&mut self, input_graph: &super::input_graph::InputGraph) -> bool;

    /// Visits a Group. Default delegates to visit_folder.
    fn visit_group(&mut self, group: &super::group::Group) -> bool {
        self.visit_folder(group)
    }

    /// Visits a GraphDocument. Default delegates to visit_folder.
    fn visit_graph_document(&mut self, document: &super::graph_document::GraphDocument) -> bool {
        self.visit_folder(document)
    }

    /// Visits a FolderElement. Default delegates to the appropriate concrete type.
    fn visit_folder_element(&mut self, element: &(dyn super::folder_element::FolderElement + Send + Sync)) -> bool {
        use std::any::Any;
        let any = element as &dyn Any;
        if let Some(group) = any.downcast_ref::<super::group::Group>() {
            return self.visit_group(group);
        }
        if let Some(input_graph) = any.downcast_ref::<super::input_graph::InputGraph>() {
            return self.visit_input_graph(input_graph);
        }
        if let Some(folder) = any.downcast_ref::<super::graph_document::GraphDocument>() {
            return self.visit_graph_document(folder);
        }
        true
    }
}