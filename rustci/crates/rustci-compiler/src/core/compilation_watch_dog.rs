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
// Rust mirror of `jdk.graal.compiler.core.CompilationWatchDog`.
// A watch dog for watching and reporting on long running compilations.

use std::time::Duration;

/// A compilation identifier — opaque type representing the compilation being watched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationIdentifier {
    /// The identifier string.
    pub id: String,
}

impl CompilationIdentifier {
    /// Creates a new compilation identifier.
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

/// Acts on compilation watch dog events.
///
/// Mirrors `CompilationWatchDog.EventHandler` interface in Java.
pub trait EventHandler {
    /// Notifies this object that a compilation is long running.
    fn on_long_compilation(
        &self,
        _compilation: &CompilationIdentifier,
        _elapsed: Duration,
        _stack_trace: &[String],
    ) {
    }

    /// Notifies this object that a compilation appears to be stuck.
    fn on_stuck_compilation(
        &self,
        _compilation: &CompilationIdentifier,
        _stack_trace: &[String],
        _stuck_time: Duration,
    ) {
    }

    /// Notifies this object that an exception occurred while watching a compilation.
    fn on_exception(&self, _exception: &dyn std::error::Error) {}
}

/// Exit code to be used when exiting a process due to a stuck compilation.
/// Mirrors `EventHandler.STUCK_COMPILATION_EXIT_CODE` in Java.
pub const STUCK_COMPILATION_EXIT_CODE: i32 = 84;

/// A shareable, default implementation of `EventHandler`.
pub struct DefaultEventHandler;

impl EventHandler for DefaultEventHandler {}

/// A watch dog for watching and reporting on long running compilations.
///
/// Mirrors `jdk.graal.compiler.core.CompilationWatchDog`.
/// Implements Runnable (run() method) and AutoCloseable (close() method).
///
/// For each compilation, a watch dog task is scheduled with an initial delay
/// of `CompilationWatchDogStartDelay` seconds. Once a scheduled watch dog task
/// starts and the compilation is still running, it will report a stack trace
/// for the compilation. Further reports will continue until the compilation
/// stops. The period between reports is doubled each time.
pub struct CompilationWatchDog {
    /// The compilation being watched.
    pub compilation: CompilationIdentifier,
    /// The delay in seconds before the watch dog starts monitoring.
    pub start_delay_secs: i32,
    /// The VM exit delay in seconds.
    pub vm_exit_delay_secs: i32,
    /// The thread being watched.
    pub watched_thread_name: Option<String>,
    /// Last recorded stack trace, if any.
    pub last_stack_trace: Vec<String>,
    /// Whether the watch dog is currently running.
    pub running: bool,
    /// Whether debug mode is enabled.
    pub debug: bool,
    /// VM exit delay in nanoseconds.
    pub vm_exit_delay_ns: u64,
}

impl CompilationWatchDog {
    /// Creates a new `CompilationWatchDog` for the given compilation.
    ///
    /// Mirrors `CompilationWatchDog.watch(CompilationIdentifier, OptionValues, boolean, EventHandler, ThreadFactory)`.
    /// Returns `None` if the delay is 0, meaning monitoring is disabled.
    pub fn watch(
        compilation: CompilationIdentifier,
        start_delay_secs: i32,
        vm_exit_delay_secs: i32,
    ) -> Option<Self> {
        if start_delay_secs > 0 {
            Some(Self {
                compilation,
                start_delay_secs,
                vm_exit_delay_secs,
                watched_thread_name: None,
                last_stack_trace: Vec::new(),
                running: false,
                debug: false,
                vm_exit_delay_ns: 0,
            })
        } else {
            None
        }
    }

    /// Creates a new `CompilationWatchDog` with full parameters.
    pub fn watch_full(
        compilation: CompilationIdentifier,
        start_delay_secs: i32,
        vm_exit_delay_secs: i32,
        debug: bool,
    ) -> Option<Self> {
        if start_delay_secs > 0 {
            Some(Self {
                compilation,
                start_delay_secs,
                vm_exit_delay_secs,
                watched_thread_name: None,
                last_stack_trace: Vec::new(),
                running: false,
                debug,
                vm_exit_delay_ns: (vm_exit_delay_secs as u64) * 1_000_000_000,
            })
        } else {
            None
        }
    }

    /// Runs the watch dog monitoring loop.
    ///
    /// Mirrors `CompilationWatchDog.run()` in Java (Runnable interface).
    /// Every 1 second, samples the stack and checks for long-running / stuck compilations.
    pub fn run(&mut self, _handler: &dyn EventHandler) {
        self.running = true;
        let mut _period_secs = self.start_delay_secs as u64;

        while self.running {
            // In the full implementation, this would:
            // 1. Sleep for the current period
            // 2. Check if the watched thread is still alive
            // 3. If alive, record the stack trace
            // 4. If the stack trace is the same as last time, report stuck
            // 5. If the stack trace differs, report long-running
            // 6. Double the period for next check
            // 7. If vm_exit_delay_ns is exceeded, exit VM

            // Store current stack trace
            self.record_stack_trace();

            // Double the period
            _period_secs *= 2;

            // Break for now — in real impl this would loop
            break;
        }

        self.running = false;
    }

    /// Closes the watch dog, stopping the monitoring task.
    ///
    /// Mirrors `CompilationWatchDog.close()` in Java (AutoCloseable interface).
    /// Calls stopCompilation internally.
    pub fn close(&mut self) {
        self.stop_compilation();
    }

    /// Stops the compilation monitoring.
    ///
    /// Mirrors the private `stopCompilation()` method in Java.
    fn stop_compilation(&mut self) {
        self.running = false;
    }

    /// Records the current stack trace of the watched thread.
    ///
    /// Mirrors the private `recordStackTrace()` method in Java.
    fn record_stack_trace(&mut self) {
        // In the full implementation, this captures the stack trace
        // of the watched thread using Thread.getStackTrace().
        self.last_stack_trace.clear();
    }

    /// Returns a string representation of this watch dog.
    ///
    /// Mirrors `toString()` in Java: `"WatchDog[" + watchedThread.getName() + "]"`
    pub fn to_string(&self) -> String {
        let thread_name = self.watched_thread_name.as_deref().unwrap_or("unknown");
        format!("WatchDog[{}]", thread_name)
    }
}

/// Options for the compilation watch dog.
///
/// Mirrors `CompilationWatchDog.Options` inner class in Java.
pub struct WatchDogOptions;

impl WatchDogOptions {
    /// Delay in seconds before watch dog monitors a compilation (0 disables monitoring).
    /// Mirrors `CompilationWatchDogStartDelay` OptionKey.
    pub const COMPILATION_WATCH_DOG_START_DELAY_DEFAULT: i32 = 0;
    /// Number of seconds after which a compilation appearing to make no progress
    /// causes the VM to exit (0 disables VM exiting).
    /// Mirrors `CompilationWatchDogVMExitDelay` OptionKey.
    pub const COMPILATION_WATCH_DOG_VM_EXIT_DELAY_DEFAULT: i32 = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watch_dog_disabled() {
        let id = CompilationIdentifier::new("test");
        let wd = CompilationWatchDog::watch(id, 0, 0);
        assert!(wd.is_none());
    }

    #[test]
    fn test_watch_dog_enabled() {
        let id = CompilationIdentifier::new("test");
        let wd = CompilationWatchDog::watch(id, 10, 5);
        assert!(wd.is_some());
        let wd = wd.unwrap();
        assert_eq!(wd.start_delay_secs, 10);
        assert_eq!(wd.vm_exit_delay_secs, 5);
    }

    #[test]
    fn test_watch_dog_full() {
        let id = CompilationIdentifier::new("test");
        let wd = CompilationWatchDog::watch_full(id, 10, 5, true);
        assert!(wd.is_some());
        let wd = wd.unwrap();
        assert!(wd.debug);
        assert_eq!(wd.vm_exit_delay_ns, 5_000_000_000);
    }

    #[test]
    fn test_watch_dog_to_string() {
        let id = CompilationIdentifier::new("test");
        let mut wd = CompilationWatchDog::watch(id, 10, 5).unwrap();
        wd.watched_thread_name = Some("compiler-1".to_string());
        assert_eq!(wd.to_string(), "WatchDog[compiler-1]");
    }

    #[test]
    fn test_watch_dog_run_and_close() {
        let id = CompilationIdentifier::new("test");
        let mut wd = CompilationWatchDog::watch(id, 10, 5).unwrap();
        let handler = DefaultEventHandler;
        wd.run(&handler);
        assert!(!wd.running); // After one iteration, it breaks
        wd.close();
        assert!(!wd.running);
    }

    #[test]
    fn test_compilation_identifier() {
        let id = CompilationIdentifier::new("my_compilation");
        assert_eq!(id.id, "my_compilation");
    }
}