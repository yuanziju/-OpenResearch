/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Provides a changed event object.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.ChangedEventProvider<T>`.
pub trait ChangedEventProvider<T> {
    /// Returns the changed event object. Should always return the same instance.
    /// Mirrors `ChangedEventProvider.getChangedEvent()`.
    fn get_changed_event(&self) -> &crate::graphio::parsing::model::changed_event::ChangedEvent<T>;
}