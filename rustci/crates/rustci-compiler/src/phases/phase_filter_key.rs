/*
 * Copyright (c) 2025, Oracle and/or its affiliates. All rights reserved.
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
// Rust mirror of `jdk.graal.compiler.phases.PhaseFilterKey`.
// An option that accepts a phase filter value.

use std::collections::HashMap;
use std::sync::Mutex;

use crate::nodes::structured_graph::StructuredGraph;

use super::base_phase::BasePhase;

/// An option that accepts a phase filter value.
/// Mirrors `jdk.graal.compiler.phases.PhaseFilterKey`.
pub struct PhaseFilterKey {
    /// The token for a phaseless graph filter. If None, phase cannot be null.
    phaseless_graph_filter_token: Option<String>,
    /// Cached parsed values.
    parsed_values: Mutex<HashMap<String, PhaseFilter>>,
}

impl PhaseFilterKey {
    /// Help message describing the syntax of a phase filter specification.
    pub const HELP: &str = "\
        A phase filter is a phase name, optionally followed by '=' and a method \
        filter. Multiple phase filters can be specified, separated by ':'. \
        A phase name is matched as a substring. \
        \
        For example: \
           PartialEscape:Loop=B.foo,A.* \
        \
        matches PartialEscapePhase for compilation of all methods and any phase \
        containing \"Loop\" in its name for compilation of B.foo as well as all \
        methods in class A. \
        \
        A phase filter specification cannot be empty. Specify \"*\" to match \
        any phase.";

    /// Creates a new PhaseFilterKey.
    pub fn new(
        _default_value: Option<String>,
        phaseless_graph_filter_token: Option<String>,
    ) -> Self {
        Self {
            phaseless_graph_filter_token,
            parsed_values: Mutex::new(HashMap::new()),
        }
    }

    /// Determines whether `phase` when applied to `graph` is matched by the filter.
    pub fn matches(
        &self,
        filter: Option<&str>,
        phase: Option<&dyn BasePhase<()>>,
        graph: &StructuredGraph,
    ) -> bool {
        let filter_str = match filter {
            Some(f) => f,
            None => return false,
        };

        let mut cache = self.parsed_values.lock().unwrap();
        let phase_filter = cache
            .entry(filter_str.to_string())
            .or_insert_with(|| PhaseFilter::parse(filter_str, &self.phaseless_graph_filter_token));

        phase_filter.matches(phase, graph)
    }
}

/// A parsed phase filter.
/// Mirrors `jdk.graal.compiler.phases.BasePhase.PhaseFilter`.
struct PhaseFilter {
    /// A map from phase pattern to graph filter.
    filters: Vec<(String, Option<String>)>,
    /// The phaseless graph filter.
    phaseless_graph_filter: Option<String>,
}

impl PhaseFilter {
    /// Creates a new PhaseFilter.
    fn new(
        filters: Vec<(String, Option<String>)>,
        phaseless_graph_filter: Option<String>,
    ) -> Self {
        Self {
            filters,
            phaseless_graph_filter,
        }
    }

    /// Determines whether this filter matches `phase` and `graph`.
    fn matches(
        &self,
        phase: Option<&dyn BasePhase<()>>,
        _graph: &StructuredGraph,
    ) -> bool {
        if phase.is_none() {
            return self.phaseless_graph_filter.is_some();
        }

        let phase_name = phase.unwrap().get_name();
        for (pattern, _graph_filter) in &self.filters {
            if phase_name.contains(pattern.as_str()) {
                return true;
            }
        }
        false
    }

    /// Parses a phase filter specification.
    fn parse(
        specification: &str,
        phaseless_graph_filter_token: &Option<String>,
    ) -> Self {
        let parts: Vec<&str> = specification.trim().split(':').collect();
        let mut filters = Vec::new();
        let mut phaseless_graph_filter = None;

        for part in parts {
            if part.contains('=') {
                let pair: Vec<&str> = part.splitn(2, '=').collect();
                if pair.len() != 2 {
                    continue;
                }
                let phase_name = pair[0].to_string();
                let graph_filter = if pair[1].is_empty() {
                    None
                } else {
                    Some(pair[1].to_string())
                };

                if let Some(ref token) = phaseless_graph_filter_token {
                    if phase_name == *token {
                        phaseless_graph_filter = graph_filter;
                        continue;
                    }
                }
                filters.push((phase_name, graph_filter));
            } else {
                let phase_name = part.to_string();
                if let Some(ref token) = phaseless_graph_filter_token {
                    if phase_name == *token {
                        phaseless_graph_filter = None;
                        continue;
                    }
                }
                filters.push((phase_name, None));
            }
        }

        Self::new(filters, phaseless_graph_filter)
    }
}