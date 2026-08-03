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
// Rust mirror of `jdk.graal.compiler.util.args.StringValue`.

use std::any::Any;
use std::fmt;

use crate::util_args::invalid_argument_exception::InvalidArgumentException;
use crate::util_args::option_value::{default_print_usage, AnyOptionValue};

/// "Parses" (effectively returns unaltered) a `String` from command line
/// arguments. Mirrors `jdk.graal.compiler.util.args.StringValue`.
pub struct StringValue {
    value: Option<String>,
    default_value: Option<String>,
    name: String,
    required: bool,
    description: String,
}

impl StringValue {
    pub fn new(name: String, help: String) -> Self {
        StringValue {
            value: None,
            default_value: None,
            name,
            required: true,
            description: help,
        }
    }

    pub fn new_with_default(name: String, default_value: String, help: String) -> Self {
        StringValue {
            value: None,
            default_value: Some(default_value),
            name,
            required: false,
            description: help,
        }
    }

    pub fn get_value(&self) -> Option<&str> {
        if let Some(ref v) = self.value {
            Some(v.as_str())
        } else {
            self.default_value.as_deref()
        }
    }
}

impl AnyOptionValue for StringValue {
    fn parse_value(&mut self, arg: Option<&str>) -> Result<bool, InvalidArgumentException> {
        let arg =
            arg.ok_or_else(|| InvalidArgumentException::new(&self.name, "no value provided"))?;
        self.value = Some(arg.to_string());
        Ok(true)
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
        default_print_usage(
            writer,
            &self.name,
            self.required,
            detailed,
            self.default_value.as_deref(),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_parsed_value(&self) -> Option<Box<dyn Any>> {
        self.value.clone().map(|v| Box::new(v) as Box<dyn Any>)
    }
}
