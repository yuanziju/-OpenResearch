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
// Rust mirror of `jdk.graal.compiler.util.args.OptionValue<T>`.
// The abstract base class is represented by the `AnyOptionValue` trait
// (for type-erased storage in collections) and helper functions for
// usage/help printing.

use std::any::Any;
use std::fmt;

use crate::util_args::invalid_argument_exception::InvalidArgumentException;

/// Indentation string used for help output. Mirrors `OptionValue.INDENT`.
pub const INDENT: &str = "  ";

/// Prints a string indented by `indent_level` levels of `INDENT`.
/// Mirrors `OptionValue.printIndented(PrintWriter, String, int)`.
pub fn print_indented(
    writer: &mut dyn fmt::Write,
    string: &str,
    indent_level: usize,
) -> fmt::Result {
    for line in string.lines() {
        for _ in 0..indent_level {
            writer.write_str(INDENT)?;
        }
        writeln!(writer, "{}", line)?;
    }
    Ok(())
}

/// Type-erased trait for option values, enabling heterogeneous storage in
/// `EconomicMap` and `Vec`. Mirrors the wildcard `OptionValue<?>` usage in
/// Java's `Command` class.
pub trait AnyOptionValue: Any {
    /// Parses the option value from the given argument. Mirrors
    /// `OptionValue.parseValue(String)`.
    fn parse_value(&mut self, arg: Option<&str>) -> Result<bool, InvalidArgumentException>;

    /// Returns true iff the option was successfully parsed. Mirrors
    /// `OptionValue.isSet()`.
    fn is_set(&self) -> bool;

    /// Returns true iff the option is required. Mirrors
    /// `OptionValue.isRequired()`.
    fn is_required(&self) -> bool;

    /// Returns the name of the option. Mirrors `OptionValue.getName()`.
    fn get_name(&self) -> &str;

    /// Returns the description of the option. Mirrors
    /// `OptionValue.getDescription()`.
    fn get_description(&self) -> &str;

    /// Clears the parsed value. Mirrors `OptionValue.clear()`.
    fn clear(&mut self);

    /// Returns the usage string. Mirrors `OptionValue.getUsage(boolean)`.
    fn get_usage(&self, detailed: bool) -> String {
        let mut s = String::new();
        let _ = self.print_usage(&mut s, detailed);
        s
    }

    /// Prints the usage of this option. Mirrors
    /// `OptionValue.printUsage(PrintWriter, boolean)`.
    fn print_usage(&self, writer: &mut dyn fmt::Write, detailed: bool) -> fmt::Result;

    /// Prints help for this option. Mirrors
    /// `OptionValue.printHelp(PrintWriter, int)`.
    fn print_help(&self, writer: &mut dyn fmt::Write, indent_level: usize) -> fmt::Result {
        print_indented(writer, self.get_description(), indent_level)
    }

    /// Enable downcasting for `instanceof` checks (e.g., `CommandGroup`,
    /// `ListValue`).
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Creates a default usage string for an option: `[name]` for optional,
/// `<name>` for required (when not detailed).
pub fn default_print_usage(
    writer: &mut dyn fmt::Write,
    name: &str,
    required: bool,
    detailed: bool,
    default_value: Option<&str>,
) -> fmt::Result {
    if required && !detailed {
        write!(writer, "<{}>", name)?;
    } else {
        write!(writer, "[{}]", name)?;
    }
    if detailed {
        if let Some(dv) = default_value {
            write!(writer, " (default: \"{}\")", dv)?;
        }
    }
    Ok(())
}

/// Creates a default usage string for an option. Convenience wrapper.
pub fn default_get_usage(
    name: &str,
    required: bool,
    detailed: bool,
    default_value: Option<&str>,
) -> String {
    let mut s = String::new();
    let _ = default_print_usage(&mut s, name, required, detailed, default_value);
    s
}
