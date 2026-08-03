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
// Rust mirror of `jdk.graal.compiler.util.args.Program`.

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use rustci_collections::pair::Pair;

use crate::util_args::command::Command;
use crate::util_args::command_parsing_exception::CommandParsingException;
use crate::util_args::help_requested_exception::HelpRequestedException;
use crate::util_args::option_value::{print_indented, AnyOptionValue, INDENT};
use crate::util_args::unknown_argument_exception::UnknownArgumentException;

/// Main entry-point of command-line parsing.
/// Mirrors `jdk.graal.compiler.util.args.Program`.
pub struct Program {
    command: Rc<RefCell<Command>>,
}

impl Program {
    pub fn new(name: String, description: String) -> Self {
        Program {
            command: Rc::new(RefCell::new(Command::new(name, description))),
        }
    }

    /// Returns a mutable reference to the underlying Command for configuration.
    pub fn command_mut(&mut self) -> std::cell::RefMut<'_, Command> {
        self.command.borrow_mut()
    }

    /// Parses the full array of command-line arguments and handles any errors.
    /// Mirrors `Program.parseAndValidate(String[], boolean)`.
    pub fn parse_and_validate(&self, args: &[String], error_is_fatal: bool) {
        let command = self.command.borrow();
        match command.parse(args, 0) {
            Ok(parsed) => {
                if parsed < args.len() {
                    let err = CommandParsingException::new(
                        &UnknownArgumentException::new(&args[parsed]),
                        Rc::clone(&self.command),
                    );
                    eprintln!("{}", err);
                    if error_is_fatal {
                        self.print_help_and_exit(1);
                    }
                }
            }
            Err(e) => {
                if let Some(_help) = e.downcast_ref::<HelpRequestedException>() {
                    self.print_help_and_exit(0);
                }
                eprintln!("{}", e);
                if error_is_fatal {
                    self.print_help_and_exit(1);
                }
            }
        }
    }

    /// Prints help and exits with the given exit code.
    /// Mirrors `Program.printHelpAndExit(int)`.
    pub fn print_help_and_exit(&self, exit_code: i32) {
        let mut output = String::new();
        let _ = self.print_help(&mut output);
        print!("{}", output);
        std::process::exit(exit_code);
    }

    /// Prints the full help message.
    /// Mirrors `Program.printHelp(PrintWriter)`.
    pub fn print_help(&self, writer: &mut dyn fmt::Write) -> fmt::Result {
        let mut positional: Vec<Rc<RefCell<Box<dyn AnyOptionValue>>>> = Vec::new();
        let mut named: Vec<Pair<String, Rc<RefCell<Box<dyn AnyOptionValue>>>>> = Vec::new();
        self.command.borrow().collect_options(&mut positional, &mut named);

        writeln!(writer)?;
        writeln!(writer, "USAGE:")?;
        writer.write_str(INDENT)?;
        self.command.borrow().print_usage(writer)?;
        writeln!(writer)?;

        if !positional.is_empty() {
            writeln!(writer)?;
            writeln!(writer, "ARGS:")?;
        }
        let mut separate = false;
        for arg in &positional {
            if separate {
                writeln!(writer)?;
            }
            let borrowed = arg.borrow();
            print_indented(writer, &borrowed.get_usage(false), 1)?;
            borrowed.print_help(writer, 2)?;
            separate = true;
        }

        if !named.is_empty() {
            writeln!(writer)?;
            writeln!(writer, "OPTIONS:")?;
        }
        separate = false;
        for pair in &named {
            if separate {
                writeln!(writer)?;
            }
            let empty = String::new();
            let name = pair.get_left().unwrap_or(&empty);
            let option = pair.get_right().unwrap();
            let opt = option.borrow();
            print_indented(
                writer,
                &format!("{} {}", name, opt.get_usage(false)),
                1,
            )?;
            opt.print_help(writer, 2)?;
            separate = true;
        }
        Ok(())
    }
}