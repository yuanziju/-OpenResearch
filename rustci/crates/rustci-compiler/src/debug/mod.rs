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
// Rust mirror of `jdk.graal.compiler.debug` package.
// Module declarations for the ported debug classes.

pub mod assertions;
pub mod counter_key;
pub mod csv_util;
pub mod debug_closeable;
pub mod debug_config;
pub mod debug_context;
pub mod debug_counter;
pub mod debug_dump_handler;
pub mod debug_filter;
pub mod debug_handler;
pub mod debug_mem_use_tracker;
pub mod debug_method_metrics;
pub mod debug_options;
pub mod debug_timer;
pub mod debug_verify_handler;
pub mod diagnostics_output_directory;
pub mod graal_error;
pub mod graal_graph_error;
pub mod graal_internal_error;
pub mod indent;
pub mod java_method_context;
pub mod key_registry;
pub mod log_stream;
pub mod mem_use_tracker_key;
pub mod scope;
pub mod timer_key;
pub mod tty;

pub use assertions::{AssertionError, Assertions};
pub use counter_key::CounterKey;
pub use csv_util::CsvUtil;
pub use debug_closeable::{DebugCloseable, DebugCloseableGuard, NoopCloseable};
pub use debug_config::{DebugConfig, DebugFilterTrait};
pub use debug_context::{DebugContext, DebugContextBuilder};
pub use debug_counter::DebugCounter;
pub use debug_dump_handler::DebugDumpHandler;
pub use debug_filter::DebugFilter;
pub use debug_handler::DebugHandler;
pub use debug_mem_use_tracker::DebugMemUseTracker;
pub use debug_method_metrics::DebugMethodMetrics;
pub use debug_options::{DebugOptionValues, DebugOptions};
pub use debug_timer::DebugTimer;
pub use debug_verify_handler::DebugVerifyHandler;
pub use diagnostics_output_directory::DiagnosticsOutputDirectory;
pub use graal_error::GraalError;
pub use graal_graph_error::GraalGraphError;
pub use graal_internal_error::GraalInternalError;
pub use indent::{indent_format, Indent, IndentGuard};
pub use java_method_context::JavaMethodContext;
pub use key_registry::{KeyInfo, KeyRegistry};
pub use log_stream::LogStream;
pub use mem_use_tracker_key::MemUseTrackerKey;
pub use scope::Scope;
pub use timer_key::TimerKey;
pub use tty::Tty;