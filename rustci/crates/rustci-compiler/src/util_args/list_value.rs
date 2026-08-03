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
// Rust mirror of `jdk.graal.compiler.util.args.ListValue<T>`.

use std::any::Any;
use std::fmt;

use crate::util_args::invalid_argument_exception::InvalidArgumentException;
use crate::util_args::option_value::AnyOptionValue;

/// Repeated option value which parses an underlying option multiple times, and
/// collects the results into a list. When parsing, this will try to consume all
/// subsequent positional arguments. Use `--` as a terminator to mark the end
/// of a list.
/// Mirrors `jdk.graal.compiler.util.args.ListValue<T>`.
pub struct ListValue {
    value: Option<Vec<Box<dyn Any>>>,
    default_value: Option<Vec<Box<dyn Any>>>,
    name: String,
    required: bool,
    description: String,
    inner: Box<dyn AnyOptionValue>,
}

impl ListValue {
    pub fn new(name: String, help: String, inner: Box<dyn AnyOptionValue>) -> Self {
        ListValue {
            value: None,
            default_value: None,
            name,
            required: true,
            description: help,
            inner,
        }
    }

    pub fn new_with_default(
        name: String,
        default_value: Vec<Box<dyn Any>>,
        help: String,
        inner: Box<dyn AnyOptionValue>,
    ) -> Self {
        ListValue {
            value: None,
            default_value: Some(default_value),
            name,
            required: false,
            description: help,
            inner,
        }
    }

    pub fn get_values(&self) -> Option<&Vec<Box<dyn Any>>> {
        if self.value.is_some() {
            self.value.as_ref()
        } else {
            self.default_value.as_ref()
        }
    }
}

impl AnyOptionValue for ListValue {
    fn parse_value(&mut self, arg: Option<&str>) -> Result<bool, InvalidArgumentException> {
        if arg.is_none() {
            return Ok(false);
        }
        let arg = arg.unwrap();
        // Attempt to parse the inner value. If it fails, terminate the list.
        let parsed = match self.inner.parse_value(Some(arg)) {
            Ok(v) => v,
            Err(_e) => {
                return Ok(false);
            }
        };
        if self.value.is_none() {
            self.value = Some(Vec::new());
        }
        if let Some(ref mut v) = self.value {
            // Store the actual parsed value from the inner option.
            // Mirroring Java: value.add(inner.value);
            if let Some(inner_value) = self.inner.get_parsed_value() {
                v.push(inner_value);
            }
        }
        Ok(parsed)
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

    fn print_usage(&self, writer: &mut dyn fmt::Write, _detailed: bool) -> fmt::Result {
        write!(writer, "[{} ...]", self.name)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_parsed_value(&self) -> Option<Box<dyn Any>> {
        // ListValue stores multiple values; can't return a single parsed value
        None
    }
}