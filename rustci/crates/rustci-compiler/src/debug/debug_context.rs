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
// Rust mirror of `jdk.graal.compiler.debug.DebugContext`.

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::debug::counter_key::CounterKey;
use crate::debug::debug_config::DebugConfig;
use crate::debug::debug_counter::DebugCounter;
use crate::debug::debug_dump_handler::DebugDumpHandler;
use crate::debug::debug_handler::DebugHandler;
use crate::debug::debug_mem_use_tracker::DebugMemUseTracker;
use crate::debug::debug_method_metrics::DebugMethodMetrics;
use crate::debug::debug_timer::DebugTimer;
use crate::debug::debug_verify_handler::DebugVerifyHandler;
use crate::debug::java_method_context::JavaMethodContext;
use crate::debug::mem_use_tracker_key::MemUseTrackerKey;
use crate::debug::scope::Scope;
use crate::debug::timer_key::TimerKey;

/// Builder for `DebugContext`. Mirrors `DebugContext.Builder`.
pub struct DebugContextBuilder {
    config: Option<Box<dyn DebugConfig>>,
    description: Option<String>,
    method_context: Option<Box<dyn JavaMethodContext>>,
}

impl DebugContextBuilder {
    /// Creates a new builder. Mirrors `DebugContext.Builder(CompilationResult)`.
    pub fn new() -> Self {
        DebugContextBuilder {
            config: None,
            description: None,
            method_context: None,
        }
    }

    /// Sets the debug configuration. Mirrors `Builder.config(DebugConfig)`.
    pub fn config(mut self, config: Box<dyn DebugConfig>) -> Self {
        self.config = Some(config);
        self
    }

    /// Sets the description. Mirrors `Builder.description(String)`.
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the method context. Mirrors `Builder.methodContext(JavaMethodContext)`.
    pub fn method_context(mut self, context: Box<dyn JavaMethodContext>) -> Self {
        self.method_context = Some(context);
        self
    }

    /// Builds the DebugContext. Mirrors `DebugContext.Builder.build()`.
    pub fn build(self) -> DebugContext {
        let config = self.config.unwrap_or_else(|| Box::new(DisabledDebugConfig));
        let handler = DebugHandler::new(config);
        DebugContext {
            handler: RefCell::new(handler),
            scope: RefCell::new(None),
            description: self.description,
            method_context: self.method_context,
            counters: RefCell::new(HashMap::new()),
            timers: RefCell::new(HashMap::new()),
            mem_use_trackers: RefCell::new(HashMap::new()),
            method_metrics: RefCell::new(None),
            dump_handlers: RefCell::new(Vec::new()),
            verify_handlers: RefCell::new(Vec::new()),
            global_properties: RefCell::new(HashMap::new()),
        }
    }
}

impl Default for DebugContextBuilder {
    fn default() -> Self {
        DebugContextBuilder::new()
    }
}

/// Mirrors `jdk.graal.compiler.debug.DebugContext`.
/// The central entry point for the Graal debug framework.
/// Manages scopes, timers, counters, memory trackers, logging, dumping, and verification.
pub struct DebugContext {
    /// The debug handler. Mirrors `DebugContext.handler`.
    handler: RefCell<DebugHandler>,
    /// The current scope. Mirrors `DebugContext.currentScope`.
    scope: RefCell<Option<Scope>>,
    /// The compilation description. Mirrors `DebugContext.description`.
    description: Option<String>,
    /// The method context. Mirrors `DebugContext.methodContext`.
    method_context: Option<Box<dyn JavaMethodContext>>,
    /// Registered counters keyed by name. Mirrors `DebugContext.counters`.
    counters: RefCell<HashMap<String, DebugCounter>>,
    /// Registered timers keyed by name. Mirrors `DebugContext.timers`.
    timers: RefCell<HashMap<String, DebugTimer>>,
    /// Registered memory trackers keyed by name. Mirrors `DebugContext.memUseTrackers`.
    mem_use_trackers: RefCell<HashMap<String, DebugMemUseTracker>>,
    /// Method-level metrics. Mirrors `DebugContext.methodMetrics`.
    method_metrics: RefCell<Option<DebugMethodMetrics>>,
    /// Dump handlers. Mirrors `DebugContext.dumpHandlers`.
    dump_handlers: RefCell<Vec<Box<dyn DebugDumpHandler>>>,
    /// Verify handlers. Mirrors `DebugContext.verifyHandlers`.
    verify_handlers: RefCell<Vec<Box<dyn DebugVerifyHandler>>>,
    /// Global properties for context sharing. Mirrors `DebugContext.globalProperties`.
    global_properties: RefCell<HashMap<String, Box<dyn Any>>>,
}

impl DebugContext {
    /// Creates a new builder. Mirrors `DebugContext.builder(CompilationResult)`.
    pub fn builder() -> DebugContextBuilder {
        DebugContextBuilder::new()
    }

    /// Creates a disabled DebugContext. Mirrors `DebugContext.disabled(CompilationResult)`.
    pub fn disabled() -> Self {
        DebugContextBuilder::new()
            .config(Box::new(DisabledDebugConfig))
            .build()
    }

    /// Returns the description. Mirrors `DebugContext.getDescription()`.
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the method context. Mirrors `DebugContext.getMethodContext()`.
    pub fn get_method_context(&self) -> Option<&dyn JavaMethodContext> {
        self.method_context.as_deref()
    }

    /// Returns whether logging is enabled. Mirrors `DebugContext.isLogEnabled()`.
    pub fn is_log_enabled(&self) -> bool {
        self.handler.borrow().get_config().is_log_enabled()
    }

    /// Returns whether dumping is enabled. Mirrors `DebugContext.isDumpEnabled()`.
    pub fn is_dump_enabled(&self) -> bool {
        self.handler.borrow().get_config().is_dump_enabled()
    }

    /// Returns whether counting is enabled. Mirrors `DebugContext.isCountEnabled()`.
    pub fn is_count_enabled(&self) -> bool {
        self.handler.borrow().get_config().is_count_enabled()
    }

    /// Returns whether timing is enabled. Mirrors `DebugContext.isTimeEnabled()`.
    pub fn is_time_enabled(&self) -> bool {
        self.handler.borrow().get_config().is_time_enabled()
    }

    /// Returns whether memory tracking is enabled. Mirrors `DebugContext.isMemUseTrackingEnabled()`.
    pub fn is_mem_use_tracking_enabled(&self) -> bool {
        self.handler.borrow().get_config().is_mem_use_tracking_enabled()
    }

    /// Returns whether verification is enabled. Mirrors `DebugContext.isVerifyEnabled()`.
    pub fn is_verify_enabled(&self) -> bool {
        self.handler.borrow().get_config().is_verify_enabled()
    }

    /// Returns the current scope. Mirrors `DebugContext.getCurrentScope()`.
    /// Returns None if no scope is currently active.
    /// Note: The returned Ref holds a borrow on the scope RefCell.
    pub fn get_current_scope(&self) -> Option<Scope> {
        // Return a clone of the current scope to avoid RefCell borrow issues.
        // This mirrors the Java pattern where the scope is accessed for inspection.
        self.scope.borrow().as_ref().map(|s| {
            // Create a lightweight copy for inspection
            Scope::new(s.get_name().to_string())
        })
    }

    /// Returns the dump level. Mirrors `DebugContext.getDumpLevel()`.
    pub fn get_dump_level(&self) -> usize {
        self.handler.borrow().get_dump_level()
    }

    /// Returns a reference to the handler. Mirrors accessing `DebugContext.handler` for
    /// getting indent, config, etc.
    pub fn handler(&self) -> std::cell::Ref<'_, DebugHandler> {
        self.handler.borrow()
    }

    /// Returns a mutable reference to the handler.
    pub fn handler_mut(&self) -> std::cell::RefMut<'_, DebugHandler> {
        self.handler.borrow_mut()
    }

    // --- Scope management ---

    /// Enters a new named scope. Mirrors `DebugContext.scope(String)`.
    /// Returns a Scope that implements DebugCloseable for try-with-resources.
    /// The returned Scope holds a reference to this DebugContext to restore
    /// the parent scope when closed.
    pub fn scope(&self, name: &str) -> Scope {
        let parent = self.scope.replace(None);
        let new_scope = if let Some(parent_scope) = parent {
            Scope::child(name.to_string(), parent_scope)
        } else {
            Scope::new(name.to_string())
        };
        // Store a reference to the new scope for tracking
        let scope_ref = Scope::new(name.to_string());
        self.scope.replace(Some(scope_ref));
        new_scope
    }

    /// Enters a new named scope with arguments. Mirrors `DebugContext.scope(String, Object...)`.
    pub fn scopev(&self, format: &str, args: &[&str]) -> Scope {
        let name = self.format_str(format, args);
        self.scope(&name)
    }

    // --- Logging ---

    /// Logs a message. Mirrors `DebugContext.log(String)`.
    pub fn log(&self, msg: &str) {
        if self.is_log_enabled() {
            self.handler.borrow_mut().log(msg);
        }
    }

    /// Logs a formatted message. Mirrors `DebugContext.log(String, Object...)`.
    pub fn logv(&self, format: &str, args: &[&str]) {
        if self.is_log_enabled() {
            self.handler.borrow_mut().logv(format, args);
        }
    }

    // --- Dumping ---

    /// Dumps an object. Mirrors `DebugContext.dump(DebugContext, Object, String)`.
    pub fn dump(&self, object: &dyn Any, name: &str) {
        if self.is_dump_enabled() {
            self.handler.borrow().dump(self, object, name);
        }
    }

    /// Dumps an object with formatted name. Mirrors `DebugContext.dump(DebugContext, Object, String, Object...)`.
    pub fn dumpv(&self, object: &dyn Any, format: &str, args: &[&str]) {
        let name = self.format_str(format, args);
        self.dump(object, &name);
    }

    // --- Verification ---

    /// Verifies an object. Mirrors `DebugContext.verify(DebugContext, Object, String)`.
    /// Returns None on success, or Some(error_message) on failure.
    pub fn verify(&self, object: &dyn Any, name: &str) -> Option<String> {
        if self.is_verify_enabled() {
            self.handler.borrow().verify(self, object, name)
        } else {
            None
        }
    }

    /// Verifies an object with formatted name. Mirrors `DebugContext.verify(DebugContext, Object, String, Object...)`.
    pub fn verifyv(&self, object: &dyn Any, format: &str, args: &[&str]) -> Option<String> {
        let name = self.format_str(format, args);
        self.verify(object, &name)
    }

    // --- Counter management ---

    /// Creates or retrieves a counter for the given key. Mirrors `DebugContext.counter(CounterKey)`.
    /// Returns a clone of the counter to avoid RefCell borrow issues.
    /// Use `increment()` and `add_to_counter()` for direct operations.
    fn get_or_create_counter(&self, name: &str) {
        let mut counters = self.counters.borrow_mut();
        if !counters.contains_key(name) {
            let counter = DebugCounter::new(name.to_string());
            counter.set_enabled(self.is_count_enabled());
            counters.insert(name.to_string(), counter);
        }
    }

    /// Increments a counter. Mirrors `DebugContext.increment(CounterKey)`.
    pub fn increment(&self, key: &CounterKey) {
        if self.is_count_enabled() {
            let name = key.get_name();
            self.get_or_create_counter(name);
            if let Some(counter) = self.counters.borrow().get(name) {
                counter.increment();
            }
        }
    }

    /// Adds a value to a counter. Mirrors `DebugContext.add(CounterKey, long)`.
    pub fn add_to_counter(&self, key: &CounterKey, delta: i64) {
        if self.is_count_enabled() {
            let name = key.get_name();
            self.get_or_create_counter(name);
            if let Some(counter) = self.counters.borrow().get(name) {
                counter.add(delta);
            }
        }
    }

    // --- Timer management ---

    /// Gets or creates a timer for the given key.
    fn get_or_create_timer(&self, name: &str) {
        let mut timers = self.timers.borrow_mut();
        if !timers.contains_key(name) {
            let timer = DebugTimer::new(name.to_string());
            timer.set_enabled(self.is_time_enabled());
            timers.insert(name.to_string(), timer);
        }
    }

    /// Starts a timer. Mirrors `DebugContext.startTimer(TimerKey)`.
    pub fn start_timer(&self, key: &TimerKey) {
        if self.is_time_enabled() {
            let name = key.get_name();
            self.get_or_create_timer(name);
            if let Some(timer) = self.timers.borrow().get(name) {
                timer.start();
            }
        }
    }

    /// Stops a timer. Mirrors `DebugContext.stopTimer(TimerKey)`.
    pub fn stop_timer(&self, key: &TimerKey) {
        if self.is_time_enabled() {
            let name = key.get_name();
            if let Some(timer) = self.timers.borrow().get(name) {
                timer.stop();
            }
        }
    }

    // --- Memory use tracker management ---

    /// Gets or creates a memory use tracker for the given key.
    fn get_or_create_mem_use_tracker(&self, name: &str) {
        let mut trackers = self.mem_use_trackers.borrow_mut();
        if !trackers.contains_key(name) {
            let tracker = DebugMemUseTracker::new(name.to_string());
            tracker.set_enabled(self.is_mem_use_tracking_enabled());
            trackers.insert(name.to_string(), tracker);
        }
    }

    /// Starts memory tracking. Mirrors `DebugContext.startMemUseTracker(MemUseTrackerKey, long)`.
    pub fn start_mem_use_tracker(&self, key: &MemUseTrackerKey, current_mem_used: i64) {
        if self.is_mem_use_tracking_enabled() {
            let name = key.get_name();
            self.get_or_create_mem_use_tracker(name);
            if let Some(tracker) = self.mem_use_trackers.borrow().get(name) {
                tracker.start(current_mem_used);
            }
        }
    }

    /// Stops memory tracking. Mirrors `DebugContext.stopMemUseTracker(MemUseTrackerKey, long)`.
    pub fn stop_mem_use_tracker(&self, key: &MemUseTrackerKey, current_mem_used: i64) {
        if self.is_mem_use_tracking_enabled() {
            let name = key.get_name();
            if let Some(tracker) = self.mem_use_trackers.borrow().get(name) {
                tracker.stop(current_mem_used);
            }
        }
    }

    // --- Method metrics ---

    /// Returns the method metrics for this context. Mirrors `DebugContext.getMethodMetrics()`.
    pub fn get_method_metrics(&self) -> Option<std::cell::Ref<'_, DebugMethodMetrics>> {
        if self.method_metrics.borrow().is_some() {
            Some(std::cell::Ref::map(self.method_metrics.borrow(), |m| {
                m.as_ref().unwrap()
            }))
        } else {
            None
        }
    }

    /// Sets method metrics. Mirrors `DebugContext.setMethodMetrics(DebugMethodMetrics)`.
    pub fn set_method_metrics(&self, metrics: DebugMethodMetrics) {
        *self.method_metrics.borrow_mut() = Some(metrics);
    }

    // --- Global properties ---

    /// Sets a global property. Mirrors `DebugContext.setGlobalProperty(String, Object)`.
    pub fn set_global_property(&self, key: &str, value: Box<dyn Any>) {
        self.global_properties.borrow_mut().insert(key.to_string(), value);
    }

    /// Gets a global property. Mirrors `DebugContext.getGlobalProperty(String)`.
    /// Returns a reference to the stored value if the property exists.
    pub fn get_global_property(&self, key: &str) -> Option<&dyn Any> {
        // We use a raw pointer approach since we can't return a Ref from RefCell.
        // The property is owned by the DebugContext and lives as long as self.
        let guard = self.global_properties.borrow();
        if let Some(value) = guard.get(key) {
            let ptr: *const Box<dyn Any> = value;
            // SAFETY: The value is stored in the HashMap owned by self.
            // The pointer is valid as long as self is alive.
            Some(unsafe { &**ptr })
        } else {
            None
        }
    }

    // --- Metrics for the current method ---

    /// Adds a value to a method-level metric. Mirrors `DebugContext.addToMetric(JavaMethod, String, long)`.
    pub fn add_to_method_metric(&self, name: &str, value: i64) {
        if let Some(ref mut metrics) = *self.method_metrics.borrow_mut() {
            metrics.add_to_metric(name, value);
        }
    }

    /// Increments a method-level metric. Mirrors `DebugContext.incrementMetric(JavaMethod, String)`.
    pub fn increment_method_metric(&self, name: &str) {
        self.add_to_method_metric(name, 1);
    }

    // --- Utility ---

    /// Formats a string with arguments. Mirrors String.format pattern.
    fn format_str(&self, format: &str, args: &[&str]) -> String {
        let mut result = format.to_string();
        for arg in args {
            if let Some(pos) = result.find("{}") {
                result.replace_range(pos..pos + 2, arg);
            }
        }
        result
    }

    /// Returns the configuration. Mirrors `DebugContext.getConfig()`.
    /// Returns a Ref guard that provides access to the config.
    /// Callers can use `handler().get_config()` for alternative access.
    pub fn get_config(&self) -> std::cell::Ref<'_, DebugHandler> {
        self.handler.borrow()
    }

    /// Returns a CSV-formatted summary of all metrics. Mirrors the CSV output.
    pub fn to_csv(&self) -> String {
        let mut csv = String::from("name;value");
        for (name, counter) in self.counters.borrow().iter() {
            csv.push('\n');
            csv.push_str(name);
            csv.push(';');
            csv.push_str(&counter.get_current_value().to_string());
        }
        for (name, timer) in self.timers.borrow().iter() {
            csv.push('\n');
            csv.push_str(name);
            csv.push(';');
            csv.push_str(&timer.get_current_value().to_string());
        }
        for (name, tracker) in self.mem_use_trackers.borrow().iter() {
            csv.push('\n');
            csv.push_str(name);
            csv.push(';');
            csv.push_str(&tracker.get_current_value().to_string());
        }
        csv
    }
}

/// A disabled debug configuration that returns false for all checks.
/// Mirrors the pattern of `DebugConfig` with all features disabled.
struct DisabledDebugConfig;

impl DebugConfig for DisabledDebugConfig {
    fn get_debug_filter(&self) -> Option<&dyn crate::debug::debug_config::DebugFilterTrait> {
        None
    }

    fn log_stream(&self) -> Option<&dyn std::io::Write> {
        None
    }

    fn dump_handlers(&self) -> &[Box<dyn DebugDumpHandler>] {
        &[]
    }

    fn verify_handlers(&self) -> &[Box<dyn DebugVerifyHandler>] {
        &[]
    }

    fn get_diagnostics_output_directory(&self) -> Option<String> {
        None
    }

    fn is_dump_enabled(&self) -> bool {
        false
    }

    fn is_log_enabled(&self) -> bool {
        false
    }

    fn is_count_enabled(&self) -> bool {
        false
    }

    fn is_time_enabled(&self) -> bool {
        false
    }

    fn is_mem_use_tracking_enabled(&self) -> bool {
        false
    }

    fn is_verify_enabled(&self) -> bool {
        false
    }

    fn is_method_meter_enabled(&self) -> bool {
        false
    }

    fn get_compilation_context(&self) -> Option<Box<dyn Any>> {
        None
    }
}