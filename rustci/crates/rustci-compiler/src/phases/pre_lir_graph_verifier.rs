/*
 * Copyright (c) 2026, Oracle and/or its affiliates. All rights reserved.
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
// Rust mirror of `jdk.graal.compiler.phases.PreLIRGraphVerifier`.

use crate::nodes::structured_graph::StructuredGraph;

use super::pre_lir_graph_verification::PreLIRGraphVerification;

/// A graph verification delegating to other graph verifications.
/// This verification succeeds only if all delegated verifications succeed.
/// Mirrors `jdk.graal.compiler.phases.PreLIRGraphVerifier`.
pub struct PreLIRGraphVerifier {
    /// The delegated verifications.
    verifications: Vec<Box<dyn PreLIRGraphVerification>>,
}

impl PreLIRGraphVerifier {
    /// Creates a new PreLIRGraphVerifier with the given verifications.
    pub fn new(verifications: Vec<Box<dyn PreLIRGraphVerification>>) -> Self {
        Self { verifications }
    }

    /// Creates a default instance with standard verifications.
    pub fn create_instance() -> Self {
        // In the full implementation, this would check options
        // and add verifications like ConstantBlindingPhase.createVerifier().
        Self {
            verifications: Vec::new(),
        }
    }
}

impl PreLIRGraphVerification for PreLIRGraphVerifier {
    fn verify(&self, graph: &StructuredGraph) -> bool {
        for verification in &self.verifications {
            if !verification.verify(graph) {
                return false;
            }
        }
        true
    }
}