/*
 * Copyright (c) 2024, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.  Oracle designates this
 * particular file as subject to the "Classpath" exception as provided
 * by Oracle in the LICENSE file that accompanied this code.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy is included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */

// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
//
// Rust mirror of `jdk.graal.compiler.phases.graph.ReentrantBlockIterator`.

/// Iterator that can re-enter blocks during traversal.
/// Mirrors `jdk.graal.compiler.phases.graph.ReentrantBlockIterator`.
pub struct ReentrantBlockIterator;

impl ReentrantBlockIterator {
    /// Creates a new reentrant block iterator.
    pub fn new() -> Self {
        Self
    }

    /// Applies the iterator to the given start block.
    pub fn apply<S: Clone>(_closure: &dyn BlockIteratorClosure<S>, _start_block: &dyn std::any::Any) -> S {
        unimplemented!("ReentrantBlockIterator::apply")
    }
}

/// Trait for closures used with ReentrantBlockIterator.
/// Mirrors `jdk.graal.compiler.phases.graph.ReentrantBlockIterator.BlockIteratorClosure`.
pub trait BlockIteratorClosure<S: Clone> {
    /// Returns the initial state.
    fn get_initial_state(&self) -> S;

    /// Processes a block and returns the new state.
    fn process_block(&self, _block: &dyn std::any::Any, current_state: S) -> S;

    /// Merges states from predecessors.
    fn merge(&self, _merge: &dyn std::any::Any, states: &[S]) -> S;

    /// Clones the state.
    fn clone_state(&self, old_state: &S) -> S;

    /// Processes a loop.
    fn process_loop(
        &self,
        _loop: &dyn std::any::Any,
        initial_state: S,
    ) -> Vec<S>;
}