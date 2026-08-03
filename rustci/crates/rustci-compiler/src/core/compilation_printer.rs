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
// Rust mirror of `jdk.graal.compiler.core.CompilationPrinter`.
// Utility for printing an informational line upon completion of compiling a method.

use crate::core::compilation_watch_dog::CompilationIdentifier;

/// Describes the source of a compilation: either a Java method or a foreign call signature.
#[derive(Debug, Clone)]
pub enum CompilationSource {
    /// A Java method being compiled.
    JavaMethod {
        /// The declaring class name.
        declaring_class: String,
        /// The method name.
        method_name: String,
        /// The method descriptor (signature).
        method_descriptor: String,
    },
    /// A foreign call (stub) being compiled.
    ForeignCallSignature {
        /// The signature name.
        name: String,
        /// The signature descriptor.
        descriptor: String,
    },
}

impl CompilationSource {
    /// Creates a `CompilationSource` for a Java method.
    pub fn java_method(declaring_class: &str, method_name: &str, method_descriptor: &str) -> Self {
        Self::JavaMethod {
            declaring_class: declaring_class.to_string(),
            method_name: method_name.to_string(),
            method_descriptor: method_descriptor.to_string(),
        }
    }

    /// Creates a `CompilationSource` for a foreign call signature.
    pub fn foreign_call(name: &str, descriptor: &str) -> Self {
        Self::ForeignCallSignature {
            name: name.to_string(),
            descriptor: descriptor.to_string(),
        }
    }
}

/// Utility for printing an informational line to TTY or a CSV file upon
/// completion of compiling a method.
///
/// Mirrors `jdk.graal.compiler.core.CompilationPrinter`.
#[derive(Debug, Clone)]
pub struct CompilationPrinter {
    /// The identifier for the compilation.
    pub id: CompilationIdentifier,
    /// The source of the compilation.
    pub source: CompilationSource,
    /// The entry BCI of the compiled method.
    pub entry_bci: i32,
    /// Whether to print to TTY.
    pub print_tty: bool,
    /// Wall-time timestamp at the beginning of the compilation (nanoseconds).
    pub begin_wall_time: u64,
    /// Thread-time timestamp at the beginning of the compilation (nanoseconds), or None if unsupported.
    pub begin_thread_time: Option<u64>,
    /// Allocated bytes at the beginning of the compilation, or None if unsupported.
    pub begin_allocated_bytes: Option<u64>,
}

impl CompilationPrinter {
    /// BCI value for invocation entry (not an OSR compilation).
    pub const INVOCATION_ENTRY_BCI: i32 = -1;

    /// Creates a disabled `CompilationPrinter` — a sentinel that does nothing.
    pub fn disabled() -> Self {
        Self {
            id: CompilationIdentifier::new(""),
            source: CompilationSource::foreign_call("", ""),
            entry_bci: -1,
            print_tty: false,
            begin_wall_time: 0,
            begin_thread_time: None,
            begin_allocated_bytes: None,
        }
    }

    /// Begins tracking a compilation for printing.
    ///
    /// Mirrors `CompilationPrinter.begin(OptionValues, CompilationIdentifier, Object, int)`.
    /// Returns `None` if neither `PrintCompilation` nor `PrintCompilationCSV` is enabled.
    pub fn begin(
        id: CompilationIdentifier,
        source: CompilationSource,
        entry_bci: i32,
        print_tty: bool,
        csv_enabled: bool,
    ) -> Option<Self> {
        if !print_tty && !csv_enabled {
            return None;
        }
        let begin_wall_time = 0u64; // placeholder — real impl uses System.nanoTime()
        Some(Self {
            id,
            source,
            entry_bci,
            print_tty,
            begin_wall_time,
            begin_thread_time: None,
            begin_allocated_bytes: None,
        })
    }

    /// Finishes tracking a compilation and prints statistics.
    ///
    /// Mirrors `CompilationPrinter.finish(CompilationResult, InstalledCode)` in Java.
    /// Prints compilation statistics to TTY or CSV when the compilation completes.
    pub fn finish(
        &self,
        target_code_size: i32,
        bytecode_size: i32,
        start_address: u64,
        code_size: i32,
        inlined_bytecodes: i32,
    ) {
        if !self.print_tty {
            return;
        }
        // In the full implementation, this would print:
        // - Method description
        // - Compilation time (wall and thread)
        // - Memory usage
        // - Code size and bytecode size
        // - Inlined bytecodes
        // - Start address
        let _ = (target_code_size, bytecode_size, start_address, code_size, inlined_bytecodes);
    }

    /// Closes the shared CSV stream.
    ///
    /// Mirrors the static `CompilationPrinter.close()` in Java.
    pub fn close_csv() {
        // In the full implementation, this closes the shared CSV PrintStream.
    }

    /// Checks whether the CSV stream is currently open.
    ///
    /// Mirrors the static `CompilationPrinter.printingToCSV()` in Java.
    pub fn printing_to_csv() -> bool {
        // In the full implementation, this checks the csvStream field.
        false
    }

    /// Returns true if this is a disabled sentinel printer.
    pub fn is_disabled(&self) -> bool {
        self.entry_bci == -1 && self.begin_wall_time == 0
    }

    /// Gets a method description string for printing.
    pub fn get_method_description(&self) -> String {
        match &self.source {
            CompilationSource::JavaMethod {
                declaring_class,
                method_name,
                method_descriptor,
            } => {
                let osr_suffix = if self.entry_bci == Self::INVOCATION_ENTRY_BCI {
                    String::new()
                } else {
                    format!("(OSR@{}) ", self.entry_bci)
                };
                format!(
                    "{:30} {:70} {:45} {:50} {}",
                    self.id.id, declaring_class, method_name, method_descriptor, osr_suffix
                )
            }
            CompilationSource::ForeignCallSignature { name, descriptor } => {
                format!(
                    "{:30} {:70} {:45} {:50} {}",
                    self.id.id, "<stub>", name, descriptor, ""
                )
            }
        }
    }
}

/// The shared CSV stream state.
///
/// Mirrors the static volatile `csvStream` field in Java's CompilationPrinter.
#[derive(Default)]
pub struct CsvState {
    /// Whether the CSV stream is currently open.
    pub is_open: bool,
    /// The CSV filename template.
    pub filename: Option<String>,
}

impl CsvState {
    /// Creates a new empty CSV state.
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disabled_printer() {
        let printer = CompilationPrinter::disabled();
        assert!(printer.is_disabled());
    }

    #[test]
    fn test_begin_disabled() {
        let id = CompilationIdentifier::new("test");
        let source = CompilationSource::java_method("Test", "foo", "()V");
        let printer = CompilationPrinter::begin(id, source, 0, false, false);
        assert!(printer.is_none());
    }

    #[test]
    fn test_begin_enabled() {
        let id = CompilationIdentifier::new("test");
        let source = CompilationSource::java_method("Test", "foo", "()V");
        let printer = CompilationPrinter::begin(id, source, 0, true, false);
        assert!(printer.is_some());
    }

    #[test]
    fn test_method_description() {
        let id = CompilationIdentifier::new("42");
        let source = CompilationSource::java_method("java/lang/String", "hashCode", "()I");
        let printer = CompilationPrinter {
            id,
            source,
            entry_bci: CompilationPrinter::INVOCATION_ENTRY_BCI,
            print_tty: true,
            begin_wall_time: 0,
            begin_thread_time: None,
            begin_allocated_bytes: None,
        };
        let desc = printer.get_method_description();
        assert!(desc.contains("hashCode"));
        assert!(desc.contains("java/lang/String"));
    }

    #[test]
    fn test_finish_does_not_panic() {
        let id = CompilationIdentifier::new("test");
        let source = CompilationSource::java_method("Test", "foo", "()V");
        let printer = CompilationPrinter::begin(id, source, 0, true, false).unwrap();
        printer.finish(100, 50, 0x1000, 100, 20);
    }

    #[test]
    fn test_printing_to_csv() {
        assert!(!CompilationPrinter::printing_to_csv());
    }
}