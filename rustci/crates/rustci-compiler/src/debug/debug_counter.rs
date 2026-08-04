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
// Rust mirror of `jdk.graal.compiler.debug.DebugCounter`.

use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};

/// Mirrors `jdk.graal.compiler.debug.DebugCounter`.
/// An accumulating counter for debug metrics. Thread-safe.
/// Java uses `AtomicLong`; Rust uses `AtomicI64`.
#[derive(Debug)]
pub struct DebugCounter {
    /// The current counter value. Mirrors `DebugCounter.value`.
    value: AtomicI64,
    /// Whether this counter is enabled. Mirrors `DebugCounter.enabled`.
    enabled: AtomicBool,
    /// The name of this counter. Mirrors `DebugCounter.name`.
    name: String,
}

impl DebugCounter {
    /// Creates a new disabled counter. Mirrors `DebugCounter(String)`.
    pub fn new(name: String) -> Self {
        DebugCounter {
            value: AtomicI64::new(0),
            enabled: AtomicBool::new(false),
            name,
        }
    }

    /// Returns the name. Mirrors `DebugCounter.getName()`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns whether counting is enabled. Mirrors `DebugCounter.isEnabled()`.
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    /// Enables or disables the counter. Mirrors `DebugCounter.setEnabled(boolean)`.
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    /// Increments the counter by 1. Mirrors `DebugCounter.increment()`.
    pub fn increment(&self) {
        if self.enabled.load(Ordering::Relaxed) {
            self.value.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Adds a value to the counter. Mirrors `DebugCounter.add(long)`.
    pub fn add(&self, delta: i64) {
        if self.enabled.load(Ordering::Relaxed) {
            self.value.fetch_add(delta, Ordering::Relaxed);
        }
    }

    /// Decrements the counter by 1. Mirrors `DebugCounter.decrement()`.
    pub fn decrement(&self) {
        if self.enabled.load(Ordering::Relaxed) {
            self.value.fetch_sub(1, Ordering::Relaxed);
        }
    }

    /// Returns the current value. Mirrors `DebugCounter.getCurrentValue()`.
    pub fn get_current_value(&self) -> i64 {
        self.value.load(Ordering::Relaxed)
    }

    /// Sets the counter to a specific value. Mirrors `DebugCounter.set(long)`.
    pub fn set(&self, value: i64) {
        if self.enabled.load(Ordering::Relaxed) {
            self.value.store(value, Ordering::Relaxed);
        }
    }

    /// Resets the counter to zero. Mirrors `DebugCounter.reset()`.
    pub fn reset(&self) {
        self.value.store(0, Ordering::Relaxed);
    }
}