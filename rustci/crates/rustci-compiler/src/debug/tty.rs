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
// Rust mirror of `jdk.graal.compiler.debug.Tty`.

use std::io::{self, Write};

/// Mirrors `jdk.graal.compiler.debug.Tty`.
/// Provides static access to the terminal output stream for debug messages.
pub struct Tty;

impl Tty {
    /// The underlying output stream. Mirrors `Tty.out`.
    /// In the Java version, this is lazily initialized. In Rust, we use stdout directly.
    fn out() -> io::Stdout {
        io::stdout()
    }

    /// The underlying error stream. Mirrors `Tty.err`.
    fn err() -> io::Stderr {
        io::stderr()
    }

    /// Prints a string to stdout. Mirrors `Tty.print(String)`.
    pub fn print(s: &str) {
        let mut out = Self::out();
        let _ = out.write_all(s.as_bytes());
        let _ = out.flush();
    }

    /// Prints a string followed by newline. Mirrors `Tty.println(String)`.
    pub fn println(s: &str) {
        let mut out = Self::out();
        let _ = out.write_all(s.as_bytes());
        let _ = out.write_all(b"\n");
        let _ = out.flush();
    }

    /// Prints a newline. Mirrors `Tty.println()`.
    pub fn println_empty() {
        let mut out = Self::out();
        let _ = out.write_all(b"\n");
        let _ = out.flush();
    }

    /// Prints to stderr. Mirrors `Tty.printErr(String)`.
    pub fn print_err(s: &str) {
        let mut err = Self::err();
        let _ = err.write_all(s.as_bytes());
        let _ = err.flush();
    }

    /// Prints to stderr with newline. Mirrors `Tty.printlnErr(String)`.
    pub fn println_err(s: &str) {
        let mut err = Self::err();
        let _ = err.write_all(s.as_bytes());
        let _ = err.write_all(b"\n");
        let _ = err.flush();
    }

    /// Formatted print to stdout. Mirrors `Tty.printf(String, Object...)`.
    pub fn printf(format: &str, args: &[&str]) {
        let mut result = format.to_string();
        for arg in args {
            if let Some(pos) = result.find("{}") {
                result.replace_range(pos..pos + 2, arg);
            }
        }
        Self::print(&result);
    }

    /// Flushes stdout. Mirrors `Tty.flush()`.
    pub fn flush() {
        let mut out = Self::out();
        let _ = out.flush();
    }
}