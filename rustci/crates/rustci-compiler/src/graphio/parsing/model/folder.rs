/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Interface for folder-like elements that contain other elements.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.Folder`.
pub trait Folder: super::folder_element::FolderElement {
    /// Returns the list of child elements.
    fn get_elements(&self) -> Vec<Box<dyn super::folder_element::FolderElement + Send + Sync>>;

    /// Removes a child element.
    fn remove_element(&mut self, element: &(dyn super::folder_element::FolderElement + Send + Sync));

    /// Adds a child element.
    fn add_element(&mut self, element: Box<dyn super::folder_element::FolderElement + Send + Sync>);

    /// Returns the changed event.
    fn get_changed_event(&self)
        -> &super::changed_event::ChangedEvent<Box<dyn Folder + Send + Sync>>;

    /// Returns true if this folder is a parent of the given child.
    fn is_parent_of(&self, child: &(dyn super::folder_element::FolderElement + Send + Sync)) -> bool {
        let mut f = child.get_parent();
        while let Some(p) = f {
            let self_ptr = self as *const _ as *const ();
            let p_ptr = p as *const _ as *const ();
            if std::ptr::eq(self_ptr, p_ptr) {
                return true;
            }
            f = p.get_parent();
        }
        false
    }
}