/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use super::input_bytecode::InputBytecode;
use super::properties::Properties;

/// Represents a method in a parsed graph.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.InputMethod`.
#[derive(Debug, Clone)]
pub struct InputMethod {
    name: String,
    short_name: String,
    bci: i32,
    bytecodes: Option<InputBytecode>,
    properties: Properties,
}

impl InputMethod {
    pub fn new(name: String, short_name: String, bci: i32) -> Self {
        InputMethod {
            name,
            short_name,
            bci,
            bytecodes: None,
            properties: Properties::new(),
        }
    }

    /// Returns the name of this method.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the short name of this method.
    pub fn get_short_name(&self) -> &str {
        &self.short_name
    }

    /// Returns the byte code index.
    pub fn get_bci(&self) -> i32 {
        self.bci
    }

    /// Returns the bytecodes, if available.
    pub fn get_bytecodes(&self) -> Option<&InputBytecode> {
        self.bytecodes.as_ref()
    }

    /// Sets the bytecodes.
    pub fn set_bytecodes(&mut self, bytecodes: InputBytecode) {
        self.bytecodes = Some(bytecodes);
    }

    /// Returns the properties of this method.
    pub fn get_properties(&self) -> &Properties {
        &self.properties
    }

    /// Returns the mutable properties of this method.
    pub fn get_properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }
}