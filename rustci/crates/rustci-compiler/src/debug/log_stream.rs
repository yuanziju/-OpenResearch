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
// Rust mirror of `jdk.graal.compiler.debug.LogStream`.

use std::io::{self, Write};

/// Mirrors `jdk.graal.compiler.debug.LogStream`.
/// A log output stream abstraction for debug output.
pub struct LogStream {
    writer: Box<dyn Write + Send>,
    indent: String,
    indent_level: usize,
    auto_flush: bool,
}

impl LogStream {
    /// Creates a new LogStream wrapping the given writer. Mirrors `LogStream(OutputStream)`.
    pub fn new(writer: Box<dyn Write + Send>) -> Self {
        LogStream {
            writer,
            indent: "  ".to_string(),
            indent_level: 0,
            auto_flush: false,
        }
    }

    /// Creates a LogStream that writes to stdout. Mirrors `LogStream.SYS_OUT`.
    pub fn sys_out() -> Self {
        LogStream::new(Box::new(io::stdout()))
    }

    /// Creates a LogStream that writes to stderr. Mirrors `LogStream.SYS_ERR`.
    pub fn sys_err() -> Self {
        LogStream::new(Box::new(io::stderr()))
    }

    /// Sets auto-flush mode. Mirrors `LogStream.setAutoFlush(boolean)`.
    pub fn set_auto_flush(&mut self, auto_flush: bool) {
        self.auto_flush = auto_flush;
    }

    /// Returns the current indentation level. Mirrors `LogStream.indentLevel()`.
    pub fn indent_level(&self) -> usize {
        self.indent_level
    }

    /// Increments indentation. Mirrors `LogStream.indent()`.
    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    /// Decrements indentation. Mirrors `LogStream.unindent()`.
    pub fn unindent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    /// Prints the current indentation. Mirrors `LogStream.printIndent()`.
    pub fn print_indent(&mut self) -> io::Result<()> {
        for _ in 0..self.indent_level {
            write!(self.writer, "{}", self.indent)?;
        }
        Ok(())
    }

    /// Prints a string. Mirrors `LogStream.print(String)`.
    pub fn print(&mut self, s: &str) -> io::Result<()> {
        self.writer.write_all(s.as_bytes())?;
        if self.auto_flush {
            self.writer.flush()?;
        }
        Ok(())
    }

    /// Prints a string followed by a newline. Mirrors `LogStream.println(String)`.
    pub fn println(&mut self, s: &str) -> io::Result<()> {
        self.print(s)?;
        self.writer.write_all(b"\n")?;
        if self.auto_flush {
            self.writer.flush()?;
        }
        Ok(())
    }

    /// Prints a newline. Mirrors `LogStream.println()`.
    pub fn println_empty(&mut self) -> io::Result<()> {
        self.writer.write_all(b"\n")?;
        if self.auto_flush {
            self.writer.flush()?;
        }
        Ok(())
    }

    /// Prints with indentation prefix. Mirrors `LogStream.printIndented(String)`.
    pub fn print_indented(&mut self, s: &str) -> io::Result<()> {
        self.print_indent()?;
        self.print(s)
    }

    /// Prints with indentation prefix followed by newline.
    pub fn println_indented(&mut self, s: &str) -> io::Result<()> {
        self.print_indent()?;
        self.println(s)
    }

    /// Formats and prints. Mirrors `LogStream.printf(String, Object...)`.
    pub fn printf(&mut self, format: &str, args: &[&str]) -> io::Result<()> {
        let mut result = format.to_string();
        for arg in args {
            if let Some(pos) = result.find("{}") {
                result.replace_range(pos..pos + 2, arg);
            }
        }
        self.print(&result)
    }

    /// Flushes the underlying stream. Mirrors `LogStream.flush()`.
    pub fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl Write for LogStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}