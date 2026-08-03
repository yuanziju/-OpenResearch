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

use std::error::Error;
use std::fmt;

/// Thrown when a `--help` flag is encountered when parsing command-line arguments.
/// Mirrors `jdk.graal.compiler.util.args.HelpRequestedException`.
#[derive(Debug)]
#[allow(dead_code)]
pub struct HelpRequestedException {
    /// The command that the user requested help for.
    command: *const (), // opaque reference to Command; actual Command is passed via the parse method
    command_name: String,
}

impl HelpRequestedException {
    pub(crate) fn new(command_name: String) -> Self {
        HelpRequestedException {
            command: std::ptr::null(),
            command_name,
        }
    }

    pub fn get_command_name(&self) -> &str {
        &self.command_name
    }
}

impl fmt::Display for HelpRequestedException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Help requested")
    }
}

impl Error for HelpRequestedException {}
