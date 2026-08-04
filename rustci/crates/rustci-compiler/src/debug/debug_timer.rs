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
// Rust mirror of `jdk.graal.compiler.debug.DebugTimer`.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::Instant;

/// Mirrors `jdk.graal.compiler.debug.DebugTimer`.
/// An accumulating timer for measuring elapsed time in debug scopes. Thread-safe.
/// Java uses `AtomicLong`/`AtomicBoolean`; Rust uses `AtomicU64`/`AtomicBool`.
#[derive(Debug)]
pub struct DebugTimer {
    /// Accumulated time in nanoseconds. Mirrors `DebugTimer.time`.
    time: AtomicU64,
    /// The time when the timer was started, if currently timing.
    /// Mirrors `DebugTimer.start`. Protected by Mutex for interior mutability.
    start: Mutex<Option<Instant>>,
    /// Whether this timer is enabled. Mirrors `DebugTimer.enabled`.
    enabled: AtomicBool,
    /// The name of this timer. Mirrors `DebugTimer.name`.
    name: String,
}

impl DebugTimer {
    /// Creates a new disabled timer. Mirrors `DebugTimer(String)`.
    pub fn new(name: String) -> Self {
        DebugTimer {
            time: AtomicU64::new(0),
            start: Mutex::new(None),
            enabled: AtomicBool::new(false),
            name,
        }
    }

    /// Returns the name. Mirrors `DebugTimer.getName()`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns whether timing is enabled. Mirrors `DebugTimer.isEnabled()`.
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    /// Enables or disables timing. Mirrors `DebugTimer.setEnabled(boolean)`.
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    /// Starts the timer. Mirrors `DebugTimer.start()`.
    /// If already started, this is a no-op.
    pub fn start(&self) {
        if self.enabled.load(Ordering::Relaxed) {
            let mut guard = self.start.lock().unwrap();
            if guard.is_none() {
                *guard = Some(Instant::now());
            }
        }
    }

    /// Stops the timer and accumulates the elapsed time. Mirrors `DebugTimer.stop()`.
    /// If not started, this is a no-op.
    pub fn stop(&self) {
        let mut guard = self.start.lock().unwrap();
        if let Some(start_time) = guard.take() {
            let elapsed = start_time.elapsed().as_nanos() as u64;
            self.time.fetch_add(elapsed, Ordering::Relaxed);
        }
    }

    /// Returns the current accumulated time in nanoseconds. Mirrors `DebugTimer.getCurrentValue()`.
    pub fn get_current_value(&self) -> u64 {
        let mut total = self.time.load(Ordering::Relaxed);
        if let Ok(guard) = self.start.try_lock() {
            if let Some(start_time) = *guard {
                total += start_time.elapsed().as_nanos() as u64;
            }
        }
        total
    }

    /// Returns the total time in seconds. Mirrors `DebugTimer.getTotalSeconds()`.
    pub fn get_total_seconds(&self) -> f64 {
        self.get_current_value() as f64 / 1_000_000_000.0
    }

    /// Resets the timer to zero. Mirrors `DebugTimer.reset()`.
    pub fn reset(&self) {
        self.time.store(0, Ordering::Relaxed);
        let mut guard = self.start.lock().unwrap();
        *guard = None;
    }
}