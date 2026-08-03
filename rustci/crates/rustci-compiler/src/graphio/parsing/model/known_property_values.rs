/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Well-known property values used in graph I/O.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.KnownPropertyValues`.
pub struct KnownPropertyValues;

impl KnownPropertyValues {
    pub const CLASS_ENDNODE: &'static str = "EndNode";
    pub const NAME_ROOT: &'static str = "Root";
    pub const NAME_START: &'static str = "Start";
}