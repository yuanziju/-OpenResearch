/*
 * Copyright (c) 2016, 2024, Oracle and/or its affiliates. All rights reserved.
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
// Rust mirror of `jdk.graal.compiler.phases.contract.NodeCostUtil`.

use super::phase_size_contract::PhaseSizeContract;

/// Estimated node size value.
#[derive(Debug, Clone, Copy)]
pub struct NodeSize {
    pub value: i32,
}

/// Utility for computing graph node costs.
/// Mirrors `jdk.graal.compiler.phases.contract.NodeCostUtil`.
pub struct NodeCostUtil;

impl NodeCostUtil {
    /// Factor to control the "imprecision" of the before-after relation when
    /// verifying phase effects.
    pub const DELTA: f64 = 0.001;

    /// Computes the total node size of an iterable of nodes.
    pub fn compute_nodes_size(node_sizes: &[NodeSize]) -> i32 {
        let mut size = 0i32;
        for ns in node_sizes {
            size += ns.value;
        }
        size
    }

    /// Computes the total graph size from a list of node sizes.
    pub fn compute_graph_size(node_sizes: &[NodeSize]) -> i32 {
        Self::compute_nodes_size(node_sizes)
    }

    /// Delta-compare two doubles.
    fn delta_compare(a: f64, b: f64, delta: f64) -> std::cmp::Ordering {
        if (a - b).abs() <= delta {
            std::cmp::Ordering::Equal
        } else {
            a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
        }
    }

    /// Verifies that a phase fulfills its size contract.
    pub fn phase_fulfills_size_contract(
        code_size_before: i32,
        code_size_after: i32,
        contract: &dyn PhaseSizeContract,
        minimal_graph_node_size_check_size: i32,
    ) {
        if code_size_before > minimal_graph_node_size_check_size {
            let code_size_increase = contract.code_size_increase() as f64;
            let graph_size_delta = code_size_before as f64 * Self::DELTA;
            if Self::delta_compare(
                code_size_after as f64,
                code_size_before as f64 * code_size_increase,
                graph_size_delta,
            ) == std::cmp::Ordering::Greater
            {
                let increase = if code_size_before == 0 {
                    code_size_after as f64
                } else {
                    code_size_after as f64 / code_size_before as f64
                };
                panic!(
                    "Phase {} expects to increase code size by at most a factor of {:.2} \
                     but an increase of {:.2} was seen (code size before: {}, after: {})",
                    contract.contractor_name(),
                    code_size_increase,
                    increase,
                    code_size_before,
                    code_size_after
                );
            }
        }
    }
}