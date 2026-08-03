/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// A single property with a name and value.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.Property<T>`.
#[derive(Debug, Clone)]
pub struct Property<T> {
    name: String,
    value: T,
}

impl<T> Property<T> {
    pub fn new(name: String, value: T) -> Self {
        if name.is_empty() {
            panic!("Property name must not be null!");
        }
        Property { name, value }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Property<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}

impl<T: PartialEq> PartialEq for Property<T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}

impl<T: std::hash::Hash> std::hash::Hash for Property<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.value.hash(state);
    }
}