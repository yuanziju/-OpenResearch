/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Listens to changed events.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.ChangedListener<T>`.
pub trait ChangedListener<T>: Send + Sync {
    /// This method is called every time a changed event is fired.
    /// Mirrors `ChangedListener.changed(T)`.
    fn changed(&self, source: &T);
}