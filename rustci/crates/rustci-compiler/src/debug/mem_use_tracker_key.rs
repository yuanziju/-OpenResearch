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
// Rust mirror of `jdk.graal.compiler.debug.MemUseTrackerKey`.

use crate::debug::key_registry::KeyRegistry;

/// Mirrors `jdk.graal.compiler.debug.DebugContext.MemUseTrackerKey`.
/// An abstract key for a named memory usage tracker metric.
#[derive(Clone, Debug)]
pub struct MemUseTrackerKey {
    /// The name of this tracker. Mirrors `AbstractKey.name`.
    name: String,
    /// The documentation string. Mirrors `AbstractKey.doc`.
    doc: String,
    /// The index of this key in the global registry. Mirrors `AbstractKey.index`.
    index: usize,
}

impl MemUseTrackerKey {
    /// Creates a new MemUseTrackerKey and registers it in the given registry.
    /// Mirrors `AbstractKey(String, String)`.
    pub fn new(name: String, doc: String, registry: &mut KeyRegistry) -> Self {
        let index = registry.register(&name, &doc);
        MemUseTrackerKey { name, doc, index }
    }

    /// Returns the name. Mirrors `AbstractKey.getName()`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the doc string. Mirrors `AbstractKey.getDoc()`.
    pub fn get_doc(&self) -> &str {
        &self.doc
    }

    /// Returns the index. Mirrors `AbstractKey.getIndex()`.
    pub fn get_index(&self) -> usize {
        self.index
    }
}

impl PartialEq for MemUseTrackerKey {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for MemUseTrackerKey {}

impl std::hash::Hash for MemUseTrackerKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}