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
// Rust mirror of `jdk.graal.compiler.util.args.CommandGroup<C extends Command>`.

use std::any::Any;
use std::cell::RefCell;
use std::fmt;

use rustci_collections::btree_economic_map::BTreeEconomicMap;
use rustci_collections::economic_map::{EconomicMap, UnmodifiableEconomicMap};

use crate::util_args::command::Command;
use crate::util_args::invalid_argument_exception::InvalidArgumentException;
use crate::util_args::option_value::{print_indented, AnyOptionValue};

/// Encapsulates a set of sub-Commands of which the user can select one
/// explicitly by name.
/// Mirrors `jdk.graal.compiler.util.args.CommandGroup<C extends Command>`.
#[allow(dead_code)]
pub struct CommandGroup {
    value: Option<RefCell<Box<Command>>>,
    default_value: Option<RefCell<Box<Command>>>,
    name: String,
    required: bool,
    description: String,
    sub_commands: RefCell<BTreeEconomicMap<String, RefCell<Box<Command>>>>,
    is_set_flag: RefCell<bool>,
}

impl CommandGroup {
    pub fn new(name: String, help: String) -> Self {
        CommandGroup {
            value: None,
            default_value: None,
            name,
            required: true,
            description: help,
            sub_commands: RefCell::new(BTreeEconomicMap::create()),
            is_set_flag: RefCell::new(false),
        }
    }

    pub fn new_with_default(name: String, default_command: Box<Command>, help: String) -> Self {
        CommandGroup {
            value: None,
            default_value: Some(RefCell::new(default_command)),
            name,
            required: false,
            description: help,
            sub_commands: RefCell::new(BTreeEconomicMap::create()),
            is_set_flag: RefCell::new(false),
        }
    }

    /// Parses and updates the selected subcommand based on args[offset].
    /// Mirrors `CommandGroup.parse(String[], int)`.
    pub fn parse(
        &self,
        args: &[String],
        offset: usize,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let arg = &args[offset];
        let sub_map = self.sub_commands.borrow();
        let sub_cell = sub_map.get(&arg.to_string());
        match sub_cell {
            Some(cell) => {
                let cmd = cell.borrow();
                let result = cmd.parse(args, offset + 1)?;
                self.is_set_flag.replace(true);
                Ok(result)
            }
            None => Err(Box::new(InvalidArgumentException::new(
                &self.name,
                &format!("no subcommand named '{}'", arg),
            ))),
        }
    }

    /// Adds a command to the set of subcommands.
    /// Mirrors `CommandGroup.addCommand(C)`.
    pub fn add_command(&mut self, command: Box<Command>) {
        let name = command.get_name().to_string();
        self.sub_commands
            .borrow_mut()
            .put(name, Some(RefCell::new(command)));
    }

    /// Returns the subcommand that was specified by the program arguments,
    /// or None if none was selected (yet).
    /// Mirrors `CommandGroup.getSelectedCommand()`.
    pub fn get_selected_command(&self) -> Option<std::cell::Ref<'_, Box<Command>>> {
        if let Some(ref cell) = self.value {
            let cmd = cell.borrow();
            return Some(std::cell::Ref::map(cmd, |_| unreachable!()));
        }
        None
    }

    /// Returns a reference to the selected command, allowing mutable access
    /// for collect_options.
    #[allow(dead_code)]
    pub(crate) fn get_selected_command_ref(&self) -> Option<&RefCell<Box<Command>>> {
        self.value.as_ref()
    }
}

impl AnyOptionValue for CommandGroup {
    fn parse_value(&mut self, _arg: Option<&str>) -> Result<bool, InvalidArgumentException> {
        panic!("unimplemented: CommandGroup.parseValue should never be called directly");
    }

    fn is_set(&self) -> bool {
        *self.is_set_flag.borrow()
    }

    fn is_required(&self) -> bool {
        self.required
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_description(&self) -> &str {
        &self.description
    }

    fn clear(&mut self) {
        self.value = None;
        self.is_set_flag.replace(false);
    }

    fn print_usage(&self, writer: &mut dyn fmt::Write, detailed: bool) -> fmt::Result {
        if let Some(ref cell) = self.value {
            let cmd = cell.borrow();
            return cmd.print_usage(writer);
        }
        if self.required && !detailed {
            write!(writer, "<{}>", self.name)
        } else {
            write!(writer, "[{}]", self.name)
        }
    }

    fn print_help(&self, writer: &mut dyn fmt::Write, indent_level: usize) -> fmt::Result {
        let mut separate = false;
        let sub_map = self.sub_commands.borrow();
        let mut cursor = sub_map.get_entries();
        while cursor.advance() {
            if separate {
                writeln!(writer)?;
            }
            let cmd = cursor.get_value().unwrap().borrow();
            print_indented(writer, cmd.get_name(), indent_level)?;
            print_indented(writer, cmd.get_description(), indent_level + 1)?;
            separate = true;
        }
        writeln!(writer)?;
        let usage = self.get_usage(false);
        print_indented(
            writer,
            &format!(
                "Pass the --help flag after {} for more help on the selected subcommand.",
                usage
            ),
            indent_level,
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
