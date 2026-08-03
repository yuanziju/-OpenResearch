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
/// Mirrors `jdk.graal.compiler.graphio.DefaultGraphBlocks`.
/// This is a singleton - use `DefaultGraphBlocks::empty()` or `empty_box()`.
pub struct DefaultGraphBlocks {
    _private: (), // private field prevents external construction
}

/// The singleton instance, mirroring `DefaultGraphBlocks.DEFAULT`.
static DEFAULT_GRAPH_BLOCKS: DefaultGraphBlocks = DefaultGraphBlocks { _private: () };

impl DefaultGraphBlocks {
    /// Returns the singleton instance, type-erased for any graph/node types.
    /// Mirrors `DefaultGraphBlocks.empty()`.
    pub fn empty<G, N>() -> &'static Self {
        &DEFAULT_GRAPH_BLOCKS
    }

    /// Returns a boxed singleton instance for use as `Box<dyn GraphBlocks<G, (), N>>`.
    /// Mirrors `DefaultGraphBlocks.empty()` which returns `GraphBlocks<G, B, N>`.
    pub fn empty_box<G: 'static, N: 'static>() -> Box<dyn GraphBlocks<G, (), N>> {
        Box::new(DefaultGraphBlocks { _private: () })
    }
}

impl Default for DefaultGraphBlocks {
    fn default() -> Self {
        DefaultGraphBlocks { _private: () }
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