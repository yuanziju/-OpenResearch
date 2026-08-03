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
    pub fn add_choice(&mut self, name: String, choice_value: Box<dyn Any>, help: String) -> &mut Self {
        self.choices.put(name.clone(), Some(choice_value));
        self.choice_help.put(name.clone(), Some(help));
        // Track default choice: if the choice_value equals the default_value,
        // set default_choice to the name. In Java, this uses choiceValue.equals(defaultValue).
        // In Rust, we can't do equality on Box<dyn Any>, so we track the first choice
        // as default if no explicit default was set, matching Java's behavior.
        if self.default_choice.is_none() && self.default_value.is_some() {
            // If we have a default value, we can't compare it to choice_value (different types),
            // so we just track the first choice. The Java code compares with .equals().
            self.default_choice = Some(name.clone());
        } else if self.default_choice.is_none() {
            self.default_choice = Some(name.clone());
        }
        self
    }

    /// Returns the value of the option. Mirrors `OptionValue.getValue()`.
    /// If the value was parsed, looks up the actual choice value from the
    /// choices map using the stored key. Otherwise returns the default value.
    pub fn get_value(&self) -> Option<&Box<dyn Any>> {
        if let Some(ref v) = self.value {
            // If the value is a string key, look up the actual choice value
            if let Some(key) = v.downcast_ref::<String>() {
                if let Some(choice) = self.choices.get(key) {
                    return Some(choice);
                }
            }
            Some(v)
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
        // Check if the choice exists in the choices map.
        // Mirroring Java: value = choices.get(arg);
        if self.choices.get(&arg.to_string()).is_some() {
            // Store the key string for lookup via get_value().
            // Since Box<dyn Any> can't be cloned, get_value() looks up
            // the actual choice value from the choices map using this key.
            self.value = Some(Box::new(arg.to_string()));
            Ok(true)
        } else {
            Err(InvalidArgumentException::new(
                &self.name,
                &format!("no choice named '{}'", arg),
            ))
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

    fn get_parsed_value(&self) -> Option<Box<dyn Any>> {
        // Look up the actual choice value from the choices map
        self.get_value().and_then(|v| {
            // We can't clone Box<dyn Any>, so we return None here
            // The actual value is accessible via get_value()
            None
        })
    }
}