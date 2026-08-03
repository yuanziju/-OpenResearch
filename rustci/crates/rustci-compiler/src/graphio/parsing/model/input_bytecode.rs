/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Represents the bytecodes of a method in a parsed graph.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.InputBytecode`.
#[derive(Debug, Clone)]
pub struct InputBytecode {
    name: String,
    bytecodes: Vec<u8>,
    bci_to_node: Vec<i32>,
    bci_to_block: Vec<i32>,
}

impl InputBytecode {
    pub fn new(name: String, bytecodes: Vec<u8>) -> Self {
        InputBytecode {
            name,
            bytecodes,
            bci_to_node: Vec::new(),
            bci_to_block: Vec::new(),
        }
    }

    /// Returns the name of the bytecode method.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the bytecodes.
    pub fn get_bytecodes(&self) -> &[u8] {
        &self.bytecodes
    }

    /// Returns the number of bytecodes.
    pub fn get_bytecodes_count(&self) -> usize {
        self.bytecodes.len()
    }

    /// Returns the node id for a given bci.
    pub fn get_bci_to_node(&self, bci: usize) -> i32 {
        if bci < self.bci_to_node.len() {
            self.bci_to_node[bci]
        } else {
            -1
        }
    }

    /// Sets the node id for a given bci.
    pub fn set_bci_to_node(&mut self, bci: usize, node_id: i32) {
        if bci >= self.bci_to_node.len() {
            self.bci_to_node.resize(bci + 1, -1);
        }
        self.bci_to_node[bci] = node_id;
    }

    /// Returns the block id for a given bci.
    pub fn get_bci_to_block(&self, bci: usize) -> i32 {
        if bci < self.bci_to_block.len() {
            self.bci_to_block[bci]
        } else {
            -1
        }
    }

    /// Sets the block id for a given bci.
    pub fn set_bci_to_block(&mut self, bci: usize, block_id: i32) {
        if bci >= self.bci_to_block.len() {
            self.bci_to_block.resize(bci + 1, -1);
        }
        self.bci_to_block[bci] = block_id;
    }
}