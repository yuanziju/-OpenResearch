/*
 * Copyright (c) 2011, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Special support for dealing with blocks in a graph.
pub trait GraphBlocks<G, B, N> {
    /// All blocks in the graph.
    fn blocks(&self, graph: &G) -> Vec<B>;

    /// Unique id of a block.
    fn block_id(&self, block: &B) -> i32;

    /// Nodes belonging to a block.
    fn block_nodes(&self, graph: &G, block: &B) -> Vec<N>;

    /// Successors of a block.
    fn block_successors(&self, block: &B) -> Vec<B>;
}
