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
// Rust mirror of `jdk.graal.compiler.phases` package.
// Phase framework: BasePhase, Phase, PhaseSuite, PhaseFilterKey,
// PlaceholderPhase, SingleRunSubphase, and supporting types.

pub mod base_phase;
pub mod class_type_sequence;
pub mod floating_guard_phase;
pub mod optimistic_optimizations;
pub mod outline_bytecode_handler_phase;
pub mod phase;
pub mod phase_filter_key;
pub mod phase_suite;
pub mod placeholder_phase;
pub mod pre_lir_graph_verification;
pub mod pre_lir_graph_verifier;
pub mod recursive_phase;
pub mod single_run_subphase;
pub mod speculative;

pub mod contract;
pub mod schedule;
pub mod tiers;
pub mod util;
pub mod common;
pub mod graph;
pub mod constantblinding;