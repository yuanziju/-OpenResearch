/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Informs that data has been loaded or removed from the collection.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.DataCollectionListener`.
pub trait DataCollectionListener: Send + Sync {
    /// Certain data has been loaded.
    fn data_loaded(&self, _evt: &super::data_collection_event::DataCollectionEvent) {}

    /// Some data has been removed from the collection.
    fn data_removed(&self, ev: &super::data_collection_event::DataCollectionEvent);
}