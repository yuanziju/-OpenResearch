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
// Rust mirror of `jdk.graal.compiler.core.CompilerThread`.
// A compiler thread is a daemon thread that runs at max priority.

/// A compiler thread is a daemon thread that runs at `Thread::MAX_PRIORITY`.
///
/// Mirrors `jdk.graal.compiler.core.CompilerThread extends Thread`.
/// In Java, CompilerThread takes a Runnable and sets priority, daemon, and name.
#[derive(Debug, Clone)]
pub struct CompilerThread {
    /// The name prefix used to construct the thread name.
    pub name_prefix: String,
    /// The thread name (constructed as namePrefix + "-" + threadId).
    pub thread_name: String,
    /// The thread ID.
    pub thread_id: u64,
    /// Whether the thread is daemon.
    pub is_daemon: bool,
    /// The thread priority.
    pub priority: i32,
    /// Whether the thread is running.
    pub running: bool,
}

impl CompilerThread {
    /// Maximum priority for compiler threads.
    pub const MAX_PRIORITY: i32 = 10;

    /// Creates a new `CompilerThread` descriptor with the given name prefix.
    ///
    /// Mirrors `CompilerThread(Runnable, String)` constructor.
    /// Sets priority to MAX_PRIORITY, daemon to true, and thread name.
    pub fn new(name_prefix: &str) -> Self {
        // In the full implementation, this would call super(r) with the Runnable.
        Self {
            name_prefix: name_prefix.to_string(),
            thread_name: name_prefix.to_string(),
            thread_id: 0,
            is_daemon: true,
            priority: Self::MAX_PRIORITY,
            running: false,
        }
    }

    /// Creates a new `CompilerThread` with a Runnable and thread ID.
    ///
    /// Mirrors the full `CompilerThread(Runnable, String)` constructor.
    pub fn with_runnable(name_prefix: &str, thread_id: u64) -> Self {
        Self {
            name_prefix: name_prefix.to_string(),
            thread_name: format!("{}-{}", name_prefix, thread_id),
            thread_id,
            is_daemon: true,
            priority: Self::MAX_PRIORITY,
            running: false,
        }
    }

    /// Runs the compiler thread.
    ///
    /// Mirrors `CompilerThread.run()` in Java.
    /// Sets the context class loader and delegates to super.run().
    pub fn run(&mut self) {
        self.running = true;
        // In the full implementation, this would:
        // 1. Set the context class loader
        // 2. Call super.run() to execute the Runnable
        // For the Rust port, we just mark as running.
    }

    /// Gets the thread name.
    pub fn get_name(&self) -> &str {
        &self.thread_name
    }

    /// Gets the thread priority.
    pub fn get_priority(&self) -> i32 {
        self.priority
    }

    /// Checks if this is a daemon thread.
    pub fn is_daemon(&self) -> bool {
        self.is_daemon
    }

    /// Checks if the thread is currently running.
    pub fn is_running(&self) -> bool {
        self.running
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_thread_new() {
        let thread = CompilerThread::new("CompilerThread");
        assert_eq!(thread.name_prefix, "CompilerThread");
        assert!(thread.is_daemon);
        assert_eq!(thread.priority, CompilerThread::MAX_PRIORITY);
    }

    #[test]
    fn test_compiler_thread_with_runnable() {
        let thread = CompilerThread::with_runnable("CompilerThread", 42);
        assert_eq!(thread.thread_name, "CompilerThread-42");
        assert_eq!(thread.thread_id, 42);
    }

    #[test]
    fn test_compiler_thread_run() {
        let mut thread = CompilerThread::new("CompilerThread");
        assert!(!thread.is_running());
        thread.run();
        assert!(thread.is_running());
    }
}