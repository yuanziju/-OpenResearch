/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::any::Any;

/// Interface for elements that can be part of a folder hierarchy.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.FolderElement`.
pub trait FolderElement: Any + Send + Sync {
    /// Returns the parent folder.
    fn get_parent(&self) -> Option<&(dyn FolderElement + Send + Sync)>;

    /// Returns the name of this element.
    fn get_name(&self) -> String;

    /// Sets the parent folder.
    fn set_parent(&mut self, parent: Option<Box<dyn FolderElement + Send + Sync>>);

    /// Returns the owner GraphDocument, if any.
    fn get_owner(&self) -> Option<&(dyn FolderElement + Send + Sync)> {
        let mut f = self.get_parent();
        while let Some(p) = f {
            let any = p as &dyn Any;
            if any.is::<super::graph_document::GraphDocument>() {
                return Some(p);
            }
            f = p.get_parent();
        }
        None
    }

    /// Opaque ID which identifies the element.
    fn get_id(&self) -> Box<dyn Any>;
}

/// Convenience macro to downcast a FolderElement to a specific type.
pub fn downcast_folder_element<'a, T: 'static>(elem: &'a (dyn FolderElement + Send + Sync)) -> Option<&'a T> {
    let any = elem as &dyn Any;
    any.downcast_ref::<T>()
}