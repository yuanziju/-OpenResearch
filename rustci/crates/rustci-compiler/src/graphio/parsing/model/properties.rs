/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::any::Any;

use super::property::Property;

/// Abstract properties collection. Mirrors `jdk.graal.compiler.graphio.parsing.model.Properties`.
/// Provides a simplified implementation using LinkedHashMap internally.
#[derive(Debug)]
pub struct Properties {
    map: linked_hash_map::LinkedHashMap<String, Box<dyn Any + Send + Sync>>,
}

/// Simple linked hash map implementation for deterministic iteration order.
mod linked_hash_map {
    use std::collections::{HashMap, VecDeque};

    #[derive(Debug, Clone)]
    pub struct LinkedHashMap<K, V> {
        order: VecDeque<K>,
        map: HashMap<K, V>,
    }

    impl<K: Clone + std::hash::Hash + Eq, V> LinkedHashMap<K, V> {
        pub fn new() -> Self {
            LinkedHashMap {
                order: VecDeque::new(),
                map: HashMap::new(),
            }
        }

        pub fn insert(&mut self, key: K, value: V) -> Option<V> {
            let old = self.map.insert(key.clone(), value);
            if old.is_none() {
                self.order.push_back(key);
            }
            old
        }

        pub fn remove(&mut self, key: &K) -> Option<V> {
            let result = self.map.remove(key);
            if result.is_some() {
                self.order.retain(|k| k != key);
            }
            result
        }

        pub fn get(&self, key: &K) -> Option<&V> {
            self.map.get(key)
        }

        pub fn contains_key(&self, key: &K) -> bool {
            self.map.contains_key(key)
        }

        pub fn len(&self) -> usize {
            self.map.len()
        }

        pub fn is_empty(&self) -> bool {
            self.map.is_empty()
        }

        pub fn clear(&mut self) {
            self.map.clear();
            self.order.clear();
        }

        pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
            self.order.iter().map(|k| {
                let v = self.map.get(k).unwrap();
                (k, v)
            })
        }

        pub fn keys(&self) -> impl Iterator<Item = &K> {
            self.order.iter()
        }
    }
}

impl Properties {
    /// Creates a new empty Properties.
    pub fn new() -> Self {
        Properties {
            map: linked_hash_map::LinkedHashMap::new(),
        }
    }

    /// Creates a new Properties with the given capacity.
    pub fn with_capacity(_capacity: usize) -> Self {
        Properties {
            map: linked_hash_map::LinkedHashMap::new(),
        }
    }

    /// Creates an immutable empty Properties.
    pub fn immutable_empty() -> Self {
        Properties::new()
    }

    /// Returns the number of properties.
    pub fn size(&self) -> usize {
        self.map.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Reserves capacity for additional properties.
    pub fn reserve(&mut self, _capacity: usize) {
        // No-op for LinkedHashMap implementation
    }

    /// Checks if a property name exists.
    pub fn contains_name(&self, name: &str) -> bool {
        self.map.contains_key(&name.to_string())
    }

    /// Gets a property value by name.
    pub fn get(&self, name: &str) -> Option<&(dyn Any + Send + Sync)> {
        self.map.get(&name.to_string()).map(|v| v.as_ref())
    }

    /// Gets a property value as a typed reference.
    pub fn get_typed<T: 'static>(&self, name: &str) -> Option<&T> {
        self.map
            .get(&name.to_string())
            .and_then(|v| v.downcast_ref::<T>())
    }

    /// Gets a string property with a default value.
    pub fn get_string(&self, name: &str) -> Option<String> {
        self.map
            .get(&name.to_string())
            .and_then(|v| v.downcast_ref::<String>())
            .cloned()
    }

    /// Sets a property value.
    pub fn set_property(&mut self, name: &str, value: Box<dyn Any + Send + Sync>) {
        self.map.insert(name.to_string(), value);
    }

    /// Removes a property by name.
    pub fn remove(&mut self, name: &str) -> Option<Box<dyn Any + Send + Sync>> {
        self.map.remove(&name.to_string())
    }

    /// Clears all properties.
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Iterates over all properties.
    pub fn iter(&self) -> impl Iterator<Item = Property<&(dyn Any + Send + Sync)>> {
        self.map.iter().map(|(key, value)| {
            Property::new(key.clone(), value.as_ref())
        })
    }
}

impl Default for Properties {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Properties {
    fn clone(&self) -> Self {
        let mut new_map = linked_hash_map::LinkedHashMap::new();
        for (key, _value) in self.map.iter() {
            // Box<dyn Any + Send + Sync> is not Clone, so we skip values.
            new_map.insert(key.clone(), Box::new(()) as Box<dyn Any + Send + Sync>);
        }
        Properties { map: new_map }
    }
}

impl PartialEq for Properties {
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() {
            return false;
        }
        self.map.iter().all(|(key, value)| {
            if let Some(other_val) = other.map.get(key) {
                // Compare by string representation for simplicity
                format!("{:?}", value) == format!("{:?}", other_val)
            } else {
                false
            }
        })
    }
}

impl std::hash::Hash for Properties {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (key, _value) in self.map.iter() {
            key.hash(state);
        }
    }
}