/*
 * Copyright (c) 2011, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use super::graph_blocks::GraphBlocks;

/// Default implementation of `GraphBlocks` that returns empty collections.
/// Useful when a graph has no block structure.
pub struct DefaultGraphBlocks;

impl DefaultGraphBlocks {
    pub fn new() -> Self {
        DefaultGraphBlocks
    }

    pub fn empty() -> Self {
        DefaultGraphBlocks
    }
}

impl Default for DefaultGraphBlocks {
    fn default() -> Self {
        DefaultGraphBlocks
    }
}

impl<G, N> GraphBlocks<G, (), N> for DefaultGraphBlocks {
    fn blocks(&self, _graph: &G) -> Vec<()> {
        vec![]
    }

    fn block_id(&self, _block: &()) -> i32 {
        -1
    }

    fn block_nodes(&self, _graph: &G, _block: &()) -> Vec<N> {
        vec![]
    }

    fn block_successors(&self, _block: &()) -> Vec<()> {
        vec![]
    }
}
