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
// Rust mirror of `jdk.graal.compiler.debug.DebugHandler`.

use crate::debug::debug_config::DebugConfig;
use crate::debug::indent::Indent;
use crate::debug::log_stream::LogStream;

/// Mirrors `jdk.graal.compiler.debug.DebugHandler`.
/// The core handler that manages debug output, dumping, and verification.
pub struct DebugHandler {
    /// The debug configuration. Mirrors `DebugHandler.config`.
    config: Box<dyn DebugConfig>,
    /// The log stream for output. Mirrors `DebugHandler.logStream`.
    log_stream: LogStream,
    /// The indent manager. Mirrors `DebugHandler.indent`.
    indent: Indent,
    /// The current dump level. Mirrors `DebugHandler.dumpLevel`.
    dump_level: usize,
    /// Whether this handler is closed. Mirrors `DebugHandler.closed`.
    closed: bool,
}

impl DebugHandler {
    /// Creates a new DebugHandler. Mirrors `DebugHandler(DebugConfig)`.
    pub fn new(config: Box<dyn DebugConfig>) -> Self {
        let log_stream = LogStream::sys_out();
        DebugHandler {
            config,
            log_stream,
            indent: Indent::new(),
            dump_level: 0,
            closed: false,
        }
    }

    /// Returns the configuration. Mirrors `DebugHandler.getConfig()`.
    pub fn get_config(&self) -> &dyn DebugConfig {
        self.config.as_ref()
    }

    /// Returns the log stream. Mirrors `DebugHandler.logStream()`.
    pub fn log_stream(&self) -> &LogStream {
        &self.log_stream
    }

    /// Returns a mutable log stream. Mirrors the pattern of getting the stream
    /// for writing.
    pub fn log_stream_mut(&mut self) -> &mut LogStream {
        &mut self.log_stream
    }

    /// Returns the indent. Mirrors `DebugHandler.indent()`.
    pub fn indent(&self) -> &Indent {
        &self.indent
    }

    /// Returns the current dump level. Mirrors `DebugHandler.getDumpLevel()`.
    pub fn get_dump_level(&self) -> usize {
        self.dump_level
    }

    /// Sets the dump level. Mirrors `DebugHandler.setDumpLevel(int)`.
    pub fn set_dump_level(&mut self, level: usize) {
        self.dump_level = level;
    }

    /// Increments the dump level. Mirrors `DebugHandler.incrementDumpLevel()`.
    pub fn increment_dump_level(&mut self) {
        self.dump_level += 1;
    }

    /// Decrements the dump level. Mirrors `DebugHandler.decrementDumpLevel()`.
    pub fn decrement_dump_level(&mut self) {
        if self.dump_level > 0 {
            self.dump_level -= 1;
        }
    }

    /// Logs a message if logging is enabled. Mirrors `DebugHandler.log(String)`.
    pub fn log(&mut self, msg: &str) {
        if self.config.is_log_enabled() {
            let _ = self.log_stream.print_indented(msg);
            let _ = self.log_stream.println_empty();
        }
    }

    /// Logs a message with arguments. Mirrors `DebugHandler.log(String, Object...)` pattern.
    pub fn logv(&mut self, format: &str, args: &[&str]) {
        if self.config.is_log_enabled() {
            let _ = self.log_stream.print_indent();
            let _ = self.log_stream.printf(format, args);
            let _ = self.log_stream.println_empty();
        }
    }

    /// Dumps an object if dumping is enabled. Mirrors `DebugHandler.dump(DebugContext, Object, String)`.
    pub fn dump(&self, debug_context: &crate::debug::debug_context::DebugContext, object: &dyn std::any::Any, name: &str) {
        if self.config.is_dump_enabled() {
            for handler in self.config.dump_handlers() {
                handler.dump(debug_context, object, name, &[]);
            }
        }
    }

    /// Verifies an object if verification is enabled.
    /// Mirrors `DebugHandler.verify(DebugContext, Object, String)`.
    pub fn verify(&self, debug_context: &crate::debug::debug_context::DebugContext, object: &dyn std::any::Any, name: &str) -> Option<String> {
        if self.config.is_verify_enabled() {
            for handler in self.config.verify_handlers() {
                if let Some(error) = handler.verify(debug_context, object, name, &[]) {
                    return Some(error);
                }
            }
        }
        None
    }

    /// Returns whether the handler is closed.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Closes the handler and releases resources. Mirrors `DebugHandler.close()`.
    pub fn close(&mut self) {
        if !self.closed {
            self.closed = true;
            let _ = self.log_stream.flush();
        }
    }
}

impl Drop for DebugHandler {
    fn drop(&mut self) {
        if !self.closed {
            let _ = self.log_stream.flush();
        }
    }
}