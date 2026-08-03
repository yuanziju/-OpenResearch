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
// Rust mirror of `jdk.graal.compiler.util.args.MultiChoiceValue<T>`.

use std::any::Any;
use std::fmt;

use rustci_collections::btree_economic_map::BTreeEconomicMap;
use rustci_collections::economic_map::{EconomicMap, UnmodifiableEconomicMap};

use crate::util_args::invalid_argument_exception::InvalidArgumentException;
use crate::util_args::option_value::{default_print_usage, print_indented, AnyOptionValue};

/// Option value with a set number of named alternatives, which are set through
/// calls to `add_choice`. Intended for enum-like options.
/// Mirrors `jdk.graal.compiler.util.args.MultiChoiceValue<T>`.
pub struct MultiChoiceValue {
    value: Option<Box<dyn Any>>,
    default_value: Option<Box<dyn Any>>,
    name: String,
    required: bool,
    description: String,
    choices: BTreeEconomicMap<String, Box<dyn Any>>,
    choice_help: BTreeEconomicMap<String, String>,
    default_choice: Option<String>,
}

impl MultiChoiceValue {
    pub fn new(name: String, help: String) -> Self {
        MultiChoiceValue {
            value: None,
            default_value: None,
            name,
            required: true,
            description: help,
            choices: BTreeEconomicMap::create(),
            choice_help: BTreeEconomicMap::create(),
            default_choice: None,
        }
    }

    pub fn new_with_default(name: String, default_value: Box<dyn Any>, help: String) -> Self {
        MultiChoiceValue {
            value: None,
            default_value: Some(default_value),
            name,
            required: false,
            description: help,
            choices: BTreeEconomicMap::create(),
            choice_help: BTreeEconomicMap::create(),
            default_choice: None,
        }
    }

    /// Adds a choice to the set of alternatives for this option.
    /// Mirrors `MultiChoiceValue.addChoice(String, T, String)`.
    pub fn add_choice(&mut self, name: String, choice_value: Box<dyn Any>, help: String) {
        self.choices.put(name.clone(), Some(choice_value));
        self.choice_help.put(name.clone(), Some(help));
        if self.default_value.is_none() {
            // no-op: first choice becomes default
        }
    }

    pub fn get_value(&self) -> Option<&Box<dyn Any>> {
        if self.value.is_some() {
            self.value.as_ref()
        } else {
            self.default_value.as_ref()
        }
    }
}

impl AnyOptionValue for MultiChoiceValue {
    fn parse_value(&mut self, arg: Option<&str>) -> Result<bool, InvalidArgumentException> {
        if arg.is_none() {
            self.value = self.default_value.take();
            return Ok(false);
        }
        let arg = arg.unwrap();
        let choice = self.choices.get(&arg.to_string());
        match choice {
            Some(_v) => {
                self.value = Some(Box::new(arg.to_string()));
                Ok(true)
            }
            None => Err(InvalidArgumentException::new(
                &self.name,
                &format!("no choice named '{}'", arg),
            )),
        }
    }

    fn is_set(&self) -> bool {
        self.value.is_some()
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
    }

    fn print_usage(&self, writer: &mut dyn fmt::Write, detailed: bool) -> fmt::Result {
        default_print_usage(writer, &self.name, self.required, false, None)?;
        if !detailed {
            return Ok(());
        }
        writer.write_str(" {")?;
        let mut sep = "";
        let mut cursor = self.choices.get_entries();
        while cursor.advance() {
            let key = cursor.get_key();
            write!(writer, "{}{}", sep, key)?;
            sep = ",";
        }
        writer.write_str("}")?;
        if let Some(ref dc) = self.default_choice {
            write!(writer, " (default: \"{}\")", dc)?;
        }
        Ok(())
    }

    fn print_help(&self, writer: &mut dyn fmt::Write, indent_level: usize) -> fmt::Result {
        print_indented(writer, &self.description, indent_level)?;
        let mut cursor = self.choice_help.get_entries();
        while cursor.advance() {
            let key = cursor.get_key();
            let value = cursor.get_value();
            let help = match value {
                Some(v) => format!("{}: {}", key, v),
                None => format!("{}: ", key),
            };
            print_indented(writer, &help, indent_level + 1)?;
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
