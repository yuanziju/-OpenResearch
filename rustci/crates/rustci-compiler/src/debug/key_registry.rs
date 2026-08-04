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
// Rust mirror of `jdk.graal.compiler.debug.KeyRegistry`.

use std::collections::HashMap;
use std::sync::Mutex;

/// Mirrors `jdk.graal.compiler.debug.KeyRegistry`.
/// Registry for debug metric keys (timers, counters, memory use trackers).
/// Provides a global namespace for retrieving and registering keys.
pub struct KeyRegistry {
    keys: HashMap<String, KeyInfo>,
}

/// Information about a registered key. Mirrors the internal tracking
/// in `KeyRegistry` for each unique key.
#[derive(Clone, Debug)]
pub struct KeyInfo {
    /// The name of the key. Mirrors `AbstractKey.name`.
    pub name: String,
    /// The doc string for the key. Mirrors `AbstractKey.doc`.
    pub doc: String,
    /// The index of this key within the registry. Mirrors `AbstractKey.index`.
    pub index: usize,
}

impl KeyRegistry {
    /// Creates a new empty key registry. Mirrors `KeyRegistry()`.
    pub fn new() -> Self {
        KeyRegistry {
            keys: HashMap::new(),
        }
    }

    /// Registers a key and returns its index. If the key is already registered,
    /// returns its existing index. Mirrors `KeyRegistry.register(AbstractKey)`.
    pub fn register(&mut self, name: &str, doc: &str) -> usize {
        if let Some(info) = self.keys.get(name) {
            return info.index;
        }
        let index = self.keys.len();
        self.keys.insert(
            name.to_string(),
            KeyInfo {
                name: name.to_string(),
                doc: doc.to_string(),
                index,
            },
        );
        index
    }

    /// Returns the number of registered keys. Mirrors `KeyRegistry.size()`.
    pub fn size(&self) -> usize {
        self.keys.len()
    }

    /// Returns the key info for a given name. Mirrors looking up by key.
    pub fn get(&self, name: &str) -> Option<&KeyInfo> {
        self.keys.get(name)
    }

    /// Returns the key info for a given index. Mirrors `KeyRegistry.get(int)`.
    pub fn get_by_index(&self, index: usize) -> Option<&KeyInfo> {
        self.keys.values().find(|info| info.index == index)
    }

    /// Returns all registered keys. Mirrors iteration over the registry.
    pub fn all_keys(&self) -> Vec<&KeyInfo> {
        let mut keys: Vec<&KeyInfo> = self.keys.values().collect();
        keys.sort_by_key(|k| k.index);
        keys
    }

    /// Clears all registered keys. Mirrors `KeyRegistry.clear()`.
    pub fn clear(&mut self) {
        self.keys.clear();
    }
}

impl Default for KeyRegistry {
    fn default() -> Self {
        KeyRegistry::new()
    }
}

/// Global key registry, protected by a mutex. Mirrors the static field
/// pattern in `KeyRegistry`.
lazy_static::lazy_static! {
    pub static ref GLOBAL_KEY_REGISTRY: Mutex<KeyRegistry> = Mutex::new(KeyRegistry::new());
}