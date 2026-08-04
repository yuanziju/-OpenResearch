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
// Rust mirror of `jdk.graal.compiler.debug.DebugMethodMetrics`.

use std::collections::HashMap;

/// Mirrors `jdk.graal.compiler.debug.DebugMethodMetrics`.
/// Tracks per-method compilation metrics for debugging and profiling.
#[derive(Debug, Clone)]
pub struct DebugMethodMetrics {
    /// The identifier for the method being tracked. Mirrors `DebugMethodMetrics.method`.
    method_name: String,
    /// The compilation id. Mirrors `DebugMethodMetrics.globalMetrics`.
    compilation_id: String,
    /// Metric values keyed by metric name. Mirrors `DebugMethodMetrics.metricValues`.
    metrics: HashMap<String, i64>,
    /// Whether metrics collection is enabled. Mirrors the enabled flag.
    enabled: bool,
}

impl DebugMethodMetrics {
    /// Creates a new metrics tracker. Mirrors `DebugMethodMetrics(JavaMethod)`.
    pub fn new(method_name: String, compilation_id: String) -> Self {
        DebugMethodMetrics {
            method_name,
            compilation_id,
            metrics: HashMap::new(),
            enabled: true,
        }
    }

    /// Returns the method name. Mirrors `DebugMethodMetrics.getMethod()`.
    pub fn get_method(&self) -> &str {
        &self.method_name
    }

    /// Returns the compilation id. Mirrors `DebugMethodMetrics.getCompilationId()`.
    pub fn get_compilation_id(&self) -> &str {
        &self.compilation_id
    }

    /// Adds a value to a metric. Mirrors `DebugMethodMetrics.addToMetric(String, long)`.
    pub fn add_to_metric(&mut self, name: &str, value: i64) {
        if !self.enabled {
            return;
        }
        let entry = self.metrics.entry(name.to_string()).or_insert(0);
        *entry += value;
    }

    /// Increments a metric by 1. Mirrors `DebugMethodMetrics.incrementMetric(String)`.
    pub fn increment_metric(&mut self, name: &str) {
        self.add_to_metric(name, 1);
    }

    /// Sets a metric to a specific value. Mirrors `DebugMethodMetrics.setMetric(String, long)`.
    pub fn set_metric(&mut self, name: &str, value: i64) {
        if !self.enabled {
            return;
        }
        self.metrics.insert(name.to_string(), value);
    }

    /// Returns the value of a metric. Mirrors `DebugMethodMetrics.getMetric(String)`.
    pub fn get_metric(&self, name: &str) -> i64 {
        self.metrics.get(name).copied().unwrap_or(0)
    }

    /// Returns all metric names. Mirrors `DebugMethodMetrics.getMetricNames()`.
    pub fn get_metric_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.metrics.keys().map(|k| k.as_str()).collect();
        names.sort();
        names
    }

    /// Returns whether metrics are enabled. Mirrors `DebugMethodMetrics.isEnabled()`.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enables metrics collection. Mirrors `DebugMethodMetrics.setEnabled(boolean)`.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Clears all metrics. Mirrors `DebugMethodMetrics.clear()`.
    pub fn clear(&mut self) {
        self.metrics.clear();
    }

    /// Returns a CSV-formatted summary of all metrics. Mirrors the CSV output pattern.
    pub fn to_csv(&self) -> String {
        let mut csv = format!("{};{}", self.method_name, self.compilation_id);
        let mut names: Vec<&String> = self.metrics.keys().collect();
        names.sort();
        for name in &names {
            csv.push(';');
            csv.push_str(&self.metrics[*name].to_string());
        }
        csv
    }
}