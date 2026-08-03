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
// Rust mirror of `jdk.graal.compiler.util.args.Command`.

use std::cell::RefCell;
use std::fmt;

use rustci_collections::btree_economic_map::BTreeEconomicMap;
use rustci_collections::economic_map::{EconomicMap, UnmodifiableEconomicMap};
use rustci_collections::pair::Pair;

use crate::util_args::command_group::CommandGroup;
use crate::util_args::help_requested_exception::HelpRequestedException;
use crate::util_args::invalid_argument_exception::InvalidArgumentException;
use crate::util_args::list_value::ListValue;
use crate::util_args::missing_argument_exception::MissingArgumentException;
use crate::util_args::option_value::AnyOptionValue;

/// Used to disambiguate where one (sub-)command ends.
/// Mirrors `Command.SEPARATOR`.
pub const SEPARATOR: &str = "--";

/// The value of a named option argument may be specified as a successive
/// program argument (e.g. --arg value) or with an equal sign (e.g. --arg=value).
/// Mirrors `Command.EQUAL_SIGN`.
pub const EQUAL_SIGN: char = '=';

/// Help flag as specified on the command-line.
/// Mirrors `Command.HELP`.
pub const HELP: &str = "--help";

/// Contains utilities to parse a set of options from command-line arguments.
/// Mirrors `jdk.graal.compiler.util.args.Command`.
pub struct Command {
    /// Map of argument names to named arguments.
    named: BTreeEconomicMap<String, RefCell<Box<dyn AnyOptionValue>>>,

    /// List of positional arguments.
    positional: Vec<RefCell<Box<dyn AnyOptionValue>>>,

    name: String,
    description: String,
}

/// Represents a reference to an option value cell, used during parsing
/// to track which option value to parse next.
enum OptionRef<'a> {
    Named(&'a RefCell<Box<dyn AnyOptionValue>>),
    Positional(&'a RefCell<Box<dyn AnyOptionValue>>),
}

impl Command {
    pub fn new(name: String, description: String) -> Self {
        Command {
            named: BTreeEconomicMap::create(),
            positional: Vec::new(),
            name,
            description,
        }
    }

    /// Appends an option to the list of positional options.
    /// Mirrors `Command.addPositional(OptionValue<T>)`.
    pub fn add_positional(&mut self, argument: Box<dyn AnyOptionValue>) {
        self.positional.push(RefCell::new(argument));
    }

    /// Adds an option to the set of named options.
    /// Mirrors `Command.addNamed(String, OptionValue<T>)`.
    pub fn add_named(&mut self, option_name: String, argument: Box<dyn AnyOptionValue>) {
        self.named.put(option_name, Some(RefCell::new(argument)));
    }

    /// Adds a subcommand group to this command.
    /// Mirrors `Command.addCommandGroup(CommandGroup<C>)`.
    pub fn add_command_group(&mut self, group: CommandGroup) {
        self.positional.push(RefCell::new(Box::new(group)));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    /// Parses an argument of the form "--option=value".
    /// Mirrors `Command.parseEqualsValue(String, int)`.
    fn parse_equals_value(
        &self,
        arg: &str,
        equal_sign_index: usize,
    ) -> Result<bool, InvalidArgumentException> {
        let option_name = &arg[..equal_sign_index];
        let value_string = &arg[equal_sign_index + 1..];
        if let Some(cell) = self.named.get(&option_name.to_string()) {
            let mut value = cell.borrow_mut();
            return value.parse_value(Some(value_string));
        }
        Ok(false)
    }

    /// Parses the full command (including subcommands, named and positional
    /// options) from command line arguments.
    /// Mirrors `Command.parse(String[], int)`.
    pub fn parse(
        &self,
        args: &[String],
        offset: usize,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let mut next_positional_arg: usize = 0;
        let mut index = offset;
        let mut current_list_value_idx: Option<usize> = None;
        loop {
            if index >= args.len() {
                break;
            }
            let arg = &args[index];
            if arg == HELP {
                return Err(Box::new(HelpRequestedException::new(self.name.clone())));
            }
            if arg == SEPARATOR {
                index += 1;
                if current_list_value_idx.is_some() {
                    current_list_value_idx = None;
                    continue;
                } else {
                    break;
                }
            }
            // Handle argument of the form option=value
            let equal_sign_index = arg.find(EQUAL_SIGN);
            if let Some(eq_idx) = equal_sign_index {
                if self.parse_equals_value(arg, eq_idx)? {
                    index += 1;
                    current_list_value_idx = None;
                    continue;
                }
            }
            // Determine which option value to parse
            let opt_ref = if let Some(named_cell) = self.named.get(&arg.clone()) {
                index += 1;
                current_list_value_idx = None;
                Some(OptionRef::Named(named_cell))
            } else if let Some(list_idx) = current_list_value_idx {
                Some(OptionRef::Positional(&self.positional[list_idx]))
            } else {
                if next_positional_arg == self.positional.len() {
                    break;
                }
                let pos_idx = next_positional_arg;
                next_positional_arg += 1;
                Some(OptionRef::Positional(&self.positional[pos_idx]))
            };

            let cell = match opt_ref {
                Some(OptionRef::Named(c)) => c,
                Some(OptionRef::Positional(c)) => c,
                None => break,
            };

            // Check if it's a CommandGroup
            {
                let result = {
                    let mut value = cell.borrow_mut();
                    if let Some(cg) = value.as_any_mut().downcast_mut::<CommandGroup>() {
                        Some(cg.parse(args, index)?)
                    } else {
                        None
                    }
                };
                if let Some(new_index) = result {
                    index = new_index;
                    continue;
                }
            }

            let arg_to_parse = if index == args.len() {
                None
            } else {
                Some(args[index].as_str())
            };

            let parsed = {
                let mut value = cell.borrow_mut();
                value.parse_value(arg_to_parse)?
            };

            if !parsed {
                if current_list_value_idx.is_some() {
                    current_list_value_idx = None;
                }
            } else {
                index += 1;
                let value = cell.borrow();
                if value.as_any().downcast_ref::<ListValue>().is_some() {
                    // Find the positional index of this cell
                    if let Some(idx) = self.positional.iter().position(|p| {
                        std::ptr::eq(
                            p.as_ptr() as *const _,
                            cell as *const RefCell<Box<dyn AnyOptionValue>>,
                        )
                    }) {
                        current_list_value_idx = Some(idx);
                    }
                }
            }
        }
        self.verify_options(next_positional_arg)?;
        Ok(index)
    }

    fn verify_options(&self, next_positional_arg: usize) -> Result<(), MissingArgumentException> {
        let mut cursor = self.named.get_entries();
        while cursor.advance() {
            let key = cursor.get_key();
            let value = cursor.get_value();
            if let Some(v) = value {
                let v = v.borrow();
                if !v.is_set() && v.is_required() {
                    return Err(MissingArgumentException::new(key));
                }
            }
        }
        for arg in next_positional_arg..self.positional.len() {
            let option = self.positional[arg].borrow();
            if option.is_required() {
                return Err(MissingArgumentException::new(option.get_name()));
            }
        }
        Ok(())
    }

    fn print_option_usage(&self, writer: &mut dyn fmt::Write) -> fmt::Result {
        let mut optional_found = false;
        let mut cursor = self.named.get_entries();
        while cursor.advance() {
            let key = cursor.get_key();
            let value = cursor.get_value();
            if let Some(v) = value {
                let v = v.borrow();
                if v.is_required() {
                    write!(writer, " ")?;
                    writer.write_str(key)?;
                    write!(writer, " ")?;
                    v.print_usage(writer, false)?;
                } else {
                    optional_found = true;
                }
            }
        }
        if optional_found {
            writer.write_str(" [OPTIONS]")?;
        }
        for option in &self.positional {
            writer.write_str(" ")?;
            option.borrow().print_usage(writer, false)?;
        }
        Ok(())
    }

    pub fn print_usage(&self, writer: &mut dyn fmt::Write) -> fmt::Result {
        writer.write_str(&self.name)?;
        self.print_option_usage(writer)
    }

    /// Collect any options in the current command and any nested subcommands
    /// into two flattened lists.
    /// Mirrors `Command.collectOptions(List, List)`.
    pub(crate) fn collect_options(
        &self,
        out_positional: &mut Vec<String>,
        out_named: &mut Vec<Pair<String, String>>,
    ) {
        for option in &self.positional {
            let opt = option.borrow();
            if let Some(cg) = opt.as_any().downcast_ref::<CommandGroup>() {
                if cg.is_set() {
                    if let Some(selected) = cg.get_selected_command() {
                        selected.collect_options(out_positional, out_named);
                    }
                } else {
                    out_positional.push(opt.get_name().to_string());
                }
            } else {
                out_positional.push(opt.get_name().to_string());
            }
        }
        let mut cursor = self.named.get_entries();
        while cursor.advance() {
            let key = cursor.get_key();
            let value = cursor.get_value();
            if let Some(v) = value {
                let v = v.borrow();
                out_named.push(Pair::create(
                    Some(key.clone()),
                    Some(v.get_name().to_string()),
                ));
            }
        }
    }
}
