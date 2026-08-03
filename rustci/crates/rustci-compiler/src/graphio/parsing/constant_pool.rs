/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

/// Dictionary of values found in the stream. Also serves as a factory to
/// create additional instances of ConstantPool.
pub struct ConstantPool {
    data: Vec<Option<Arc<dyn std::any::Any + Send + Sync>>>,
    entries_added: usize,
}

impl ConstantPool {
    pub fn new() -> Self {
        ConstantPool {
            data: Vec::new(),
            entries_added: 0,
        }
    }

    pub fn with_data(data: Vec<Option<Arc<dyn std::any::Any + Send + Sync>>>) -> Self {
        ConstantPool {
            data,
            entries_added: 0,
        }
    }

    pub fn add_pool_entry(
        &mut self,
        index: usize,
        obj: Arc<dyn std::any::Any + Send + Sync>,
    ) -> Arc<dyn std::any::Any + Send + Sync> {
        while self.data.len() <= index {
            self.data.push(None);
        }
        self.entries_added += 1;
        self.data[index] = Some(obj.clone());
        obj
    }

    pub fn get(&self, index: usize) -> Option<&Arc<dyn std::any::Any + Send + Sync>> {
        self.data.get(index).and_then(|o| o.as_ref())
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn snapshot(&self) -> Vec<Option<Arc<dyn std::any::Any + Send + Sync>>> {
        self.data.clone()
    }

    pub fn swap(
        &mut self,
        replacement_data: Vec<Option<Arc<dyn std::any::Any + Send + Sync>>>,
    ) -> ConstantPool {
        let copy = ConstantPool::with_data(self.data.clone());
        self.data = replacement_data;
        copy
    }

    pub fn copy(&self) -> ConstantPool {
        ConstantPool::with_data(self.data.clone())
    }

    pub fn restart(&self) -> ConstantPool {
        ConstantPool::new()
    }

    pub fn entries_added(&self) -> usize {
        self.entries_added
    }

    pub fn copy_data(&self) -> Vec<Option<Arc<dyn std::any::Any + Send + Sync>>> {
        self.data.clone()
    }
}

impl Default for ConstantPool {
    fn default() -> Self {
        ConstantPool::new()
    }
}

/// Global counter for total entries added across all pools.
pub static TOTAL_ENTRIES: AtomicUsize = AtomicUsize::new(0);
