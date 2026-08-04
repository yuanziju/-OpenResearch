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
// Rust mirror of `jdk.graal.compiler.debug.DebugMemUseTracker`.

use std::cell::Cell;

/// Mirrors `jdk.graal.compiler.debug.DebugMemUseTracker`.
/// Tracks memory usage over a debug scope.
#[derive(Debug)]
pub struct DebugMemUseTracker {
    /// The current tracked memory value in bytes. Mirrors `DebugMemUseTracker.value`.
    value: Cell<i64>,
    /// The baseline memory when tracking started. Mirrors `DebugMemUseTracker.start`.
    start: Cell<i64>,
    /// Whether tracking is active. Mirrors the tracking state.
    tracking: Cell<bool>,
    /// Whether this tracker is enabled. Mirrors `DebugMemUseTracker.enabled`.
    enabled: Cell<bool>,
    /// The name of this tracker. Mirrors `DebugMemUseTracker.name`.
    name: String,
}

impl DebugMemUseTracker {
    /// Creates a new disabled tracker. Mirrors `DebugMemUseTracker(String)`.
    pub fn new(name: String) -> Self {
        DebugMemUseTracker {
            value: Cell::new(0),
            start: Cell::new(0),
            tracking: Cell::new(false),
            enabled: Cell::new(false),
            name,
        }
    }

    /// Returns the name. Mirrors `DebugMemUseTracker.getName()`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns whether tracking is enabled. Mirrors `DebugMemUseTracker.isEnabled()`.
    pub fn is_enabled(&self) -> bool {
        self.enabled.get()
    }

    /// Enables or disables tracking. Mirrors `DebugMemUseTracker.setEnabled(boolean)`.
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.set(enabled);
    }

    /// Starts tracking memory usage. Mirrors `DebugMemUseTracker.start(long)`.
    /// `current_mem_used` is the current memory usage in bytes.
    pub fn start(&self, current_mem_used: i64) {
        if self.enabled.get() && !self.tracking.get() {
            self.start.set(current_mem_used);
            self.tracking.set(true);
        }
    }

    /// Stops tracking and records the delta. Mirrors `DebugMemUseTracker.stop(long)`.
    /// `current_mem_used` is the current memory usage in bytes.
    pub fn stop(&self, current_mem_used: i64) {
        if self.tracking.get() {
            let delta = current_mem_used - self.start.get();
            self.value.set(self.value.get() + delta);
            self.tracking.set(false);
        }
    }

    /// Returns the current accumulated memory delta. Mirrors `DebugMemUseTracker.getCurrentValue()`.
    pub fn get_current_value(&self) -> i64 {
        self.value.get()
    }

    /// Returns the current value in kilobytes. Mirrors `DebugMemUseTracker.getCurrentValueKB()`.
    pub fn get_current_value_kb(&self) -> f64 {
        self.value.get() as f64 / 1024.0
    }

    /// Returns the current value in megabytes. Mirrors `DebugMemUseTracker.getCurrentValueMB()`.
    pub fn get_current_value_mb(&self) -> f64 {
        self.value.get() as f64 / (1024.0 * 1024.0)
    }

    /// Resets the tracker to zero. Mirrors `DebugMemUseTracker.reset()`.
    pub fn reset(&self) {
        self.value.set(0);
        self.start.set(0);
        self.tracking.set(false);
    }
}