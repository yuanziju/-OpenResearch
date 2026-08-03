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
// Rust mirror of `jdk.graal.compiler.util.args` package.
// Module declarations for the 16 ported classes.

pub mod boolean_value;
pub mod command;
pub mod command_group;
pub mod command_parsing_exception;
pub mod double_value;
pub mod flag;
pub mod help_requested_exception;
pub mod integer_value;
pub mod invalid_argument_exception;
pub mod list_value;
pub mod missing_argument_exception;
pub mod multi_choice_value;
pub mod option_value;
pub mod program;
pub mod string_value;
pub mod unknown_argument_exception;

pub use boolean_value::BooleanValue;
pub use command::Command;
pub use command_group::CommandGroup;
pub use command_parsing_exception::CommandParsingException;
pub use double_value::DoubleValue;
pub use flag::Flag;
pub use help_requested_exception::HelpRequestedException;
pub use integer_value::IntegerValue;
pub use invalid_argument_exception::InvalidArgumentException;
pub use list_value::ListValue;
pub use missing_argument_exception::MissingArgumentException;
pub use multi_choice_value::MultiChoiceValue;
pub use option_value::{print_indented, AnyOptionValue, INDENT};
pub use program::Program;
pub use string_value::StringValue;
pub use unknown_argument_exception::UnknownArgumentException;
