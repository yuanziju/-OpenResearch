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
// Rust mirror of `jdk.graal.compiler.util.args.Flag`.

use std::any::Any;
use std::fmt;

use crate::util_args::invalid_argument_exception::InvalidArgumentException;
use crate::util_args::option_value::AnyOptionValue;

/// A boolean flag option value that is `false` when not present in the program
/// arguments and `true` when it is present.
/// Mirrors `jdk.graal.compiler.util.args.Flag`.
pub struct Flag {
    value: Option<bool>,
    description: String,
}

impl Flag {
    pub fn new(help: String) -> Self {
        Flag {
            value: None,
            description: help,
        }
    }

    pub fn get_value(&self) -> bool {
        self.value.unwrap_or(false)
    }
}

impl AnyOptionValue for Flag {
    fn parse_value(&mut self, _arg: Option<&str>) -> Result<bool, InvalidArgumentException> {
        self.value = Some(true);
        Ok(false)
    }

    fn is_set(&self) -> bool {
        self.value.is_some()
    }

    fn is_required(&self) -> bool {
        false
    }

    fn get_name(&self) -> &str {
        ""
    }

    fn get_description(&self) -> &str {
        &self.description
    }

    fn clear(&mut self) {
        self.value = Some(false);
    }

    fn print_usage(&self, _writer: &mut dyn fmt::Write, _detailed: bool) -> fmt::Result {
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
