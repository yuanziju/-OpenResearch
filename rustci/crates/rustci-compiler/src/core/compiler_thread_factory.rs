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
// Rust mirror of `jdk.graal.compiler.core.CompilerThreadFactory`.
// Facility for creating compiler threads.

use crate::core::compiler_thread::CompilerThread;

/// Trait for thread factories.
///
/// Mirrors `java.util.concurrent.ThreadFactory` interface in Java.
/// CompilerThreadFactory implements this trait.
pub trait ThreadFactory {
    /// Creates a new thread for the given Runnable.
    fn new_thread(&self) -> CompilerThread;
}

/// Facility for creating compiler threads.
///
/// Mirrors `jdk.graal.compiler.core.CompilerThreadFactory implements ThreadFactory`.
#[derive(Debug, Clone)]
pub struct CompilerThreadFactory {
    /// The name prefix for compiler threads.
    pub thread_name_prefix: String,
    /// Counter for thread IDs.
    pub thread_counter: u64,
}

impl CompilerThreadFactory {
    /// Creates a new `CompilerThreadFactory` with the given name prefix.
    pub fn new(thread_name_prefix: &str) -> Self {
        Self {
            thread_name_prefix: thread_name_prefix.to_string(),
            thread_counter: 0,
        }
    }
}

impl ThreadFactory for CompilerThreadFactory {
    /// Creates a new `CompilerThread` with the stored name prefix.
    ///
    /// Mirrors `newThread(Runnable r)` in Java.
    /// Returns `new CompilerThread(r, threadNamePrefix)`.
    fn new_thread(&self) -> CompilerThread {
        // In the full implementation, this creates a new CompilerThread
        // with the given Runnable and the thread name prefix.
        CompilerThread::new(&self.thread_name_prefix)
    }
}

impl CompilerThreadFactory {
    /// Creates a new `CompilerThread` with a unique thread ID.
    pub fn new_thread_with_id(&mut self) -> CompilerThread {
        let id = self.thread_counter;
        self.thread_counter += 1;
        CompilerThread::with_runnable(&self.thread_name_prefix, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_thread_factory_new() {
        let factory = CompilerThreadFactory::new("GraalCompiler");
        assert_eq!(factory.thread_name_prefix, "GraalCompiler");
    }

    #[test]
    fn test_thread_factory_trait() {
        let factory = CompilerThreadFactory::new("GraalCompiler");
        let thread = factory.new_thread();
        assert_eq!(thread.name_prefix, "GraalCompiler");
    }

    #[test]
    fn test_new_thread_with_id() {
        let mut factory = CompilerThreadFactory::new("GraalCompiler");
        let thread1 = factory.new_thread_with_id();
        assert_eq!(thread1.thread_id, 0);
        let thread2 = factory.new_thread_with_id();
        assert_eq!(thread2.thread_id, 1);
    }
}