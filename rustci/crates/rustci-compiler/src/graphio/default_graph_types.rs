/*
 * Copyright (c) 2011, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use super::graph_types::GraphTypes;

/// Default implementation of `GraphTypes` that handles `Enum`-like and
/// `Class`-like objects. In Rust, custom enum recognition is delegated to
/// the user-provided implementation; this default simply returns `None`/`-1`
/// for all queries.
/// Mirrors `jdk.graal.compiler.graphio.DefaultGraphTypes`.
/// This is a singleton - use `DefaultGraphTypes::default_instance()` or `default_instance_box()`.
pub struct DefaultGraphTypes {
    _private: (), // private field prevents external construction
}

/// The singleton instance, mirroring `DefaultGraphTypes.DEFAULT`.
static DEFAULT_GRAPH_TYPES: DefaultGraphTypes = DefaultGraphTypes { _private: () };

impl DefaultGraphTypes {
    /// Returns the singleton instance. Mirrors `DefaultGraphTypes.DEFAULT`.
    pub fn default_instance() -> &'static Self {
        &DEFAULT_GRAPH_TYPES
    }

    /// Returns a boxed singleton instance for use as `Box<dyn GraphTypes>`.
    /// Mirrors `DefaultGraphTypes.DEFAULT` which is `static final GraphTypes DEFAULT`.
    pub fn default_instance_box() -> Box<dyn GraphTypes> {
        Box::new(DefaultGraphTypes { _private: () })
    }
}

impl Default for DefaultGraphTypes {
    fn default() -> Self {
        DefaultGraphTypes { _private: () }
    }
}

impl GraphTypes for DefaultGraphTypes {
    fn enum_class(&self, _enum_value: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn enum_ordinal(&self, _obj: &dyn std::any::Any) -> i32 {
        -1
    }

    fn enum_type_values(&self, _maybe_enum_class: &dyn std::any::Any) -> Option<Vec<String>> {
        None
    }

    fn type_name(&self, _maybe_class: &dyn std::any::Any) -> Option<String> {
        None
    }
}