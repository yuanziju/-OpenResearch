/*
 * Copyright (c) 2012, 2024, Oracle and/or its affiliates. All rights reserved.
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
// Rust mirror of `jdk.graal.compiler.phases.OptimisticOptimizations`.

use std::collections::BTreeSet;

/// Enumerates the possible optimistic optimizations.
/// Mirrors `jdk.graal.compiler.phases.OptimisticOptimizations.Optimization`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Optimization {
    /// Remove code that is never executed.
    RemoveNeverExecutedCode,
    /// Use type-checked inlining.
    UseTypeCheckedInlining,
    /// Use type-check hints.
    UseTypeCheckHints,
    /// Use exception probability for operations.
    UseExceptionProbabilityForOperations,
    /// Use exception probability.
    UseExceptionProbability,
    /// Use loop limit checks.
    UseLoopLimitChecks,
}

/// Controls which optimistic optimizations are enabled.
/// Mirrors `jdk.graal.compiler.phases.OptimisticOptimizations`.
#[derive(Debug, Clone)]
pub struct OptimisticOptimizations {
    /// The set of enabled optimizations.
    enabled_opts: BTreeSet<Optimization>,
}

impl OptimisticOptimizations {
    /// All optimizations enabled.
    pub fn all() -> Self {
        let mut opts = BTreeSet::new();
        opts.insert(Optimization::RemoveNeverExecutedCode);
        opts.insert(Optimization::UseTypeCheckedInlining);
        opts.insert(Optimization::UseTypeCheckHints);
        opts.insert(Optimization::UseExceptionProbabilityForOperations);
        opts.insert(Optimization::UseExceptionProbability);
        opts.insert(Optimization::UseLoopLimitChecks);
        Self { enabled_opts: opts }
    }

    /// No optimizations enabled.
    pub fn none() -> Self {
        Self {
            enabled_opts: BTreeSet::new(),
        }
    }

    /// Creates a new instance with the given set of enabled optimizations.
    pub fn new(enabled_opts: BTreeSet<Optimization>) -> Self {
        Self { enabled_opts }
    }

    /// Creates from profiling info and options.
    /// This is a simplified version that accepts a pre-built set.
    pub fn from_enabled(enabled: &[Optimization]) -> Self {
        let mut opts = BTreeSet::new();
        opts.insert(Optimization::UseExceptionProbabilityForOperations);
        for opt in enabled {
            opts.insert(*opt);
        }
        Self { enabled_opts: opts }
    }

    /// Removes the given optimizations from this set.
    pub fn remove(&self, optimizations: &[Optimization]) -> Self {
        let mut new_opts = self.enabled_opts.clone();
        for opt in optimizations {
            new_opts.remove(opt);
        }
        Self {
            enabled_opts: new_opts,
        }
    }

    /// Whether remove-never-executed-code is enabled.
    pub fn remove_never_executed_code(&self, option_enabled: bool) -> bool {
        option_enabled && self.enabled_opts.contains(&Optimization::RemoveNeverExecutedCode)
    }

    /// Whether type-check hints are enabled.
    pub fn use_type_check_hints(&self, option_enabled: bool) -> bool {
        option_enabled && self.enabled_opts.contains(&Optimization::UseTypeCheckHints)
    }

    /// Whether monomorphic call inlining is enabled.
    pub fn inline_monomorphic_calls(&self, option_enabled: bool) -> bool {
        option_enabled && self.enabled_opts.contains(&Optimization::UseTypeCheckedInlining)
    }

    /// Whether polymorphic call inlining is enabled.
    pub fn inline_polymorphic_calls(&self, option_enabled: bool) -> bool {
        option_enabled && self.enabled_opts.contains(&Optimization::UseTypeCheckedInlining)
    }

    /// Whether megamorphic call inlining is enabled.
    pub fn inline_megamorphic_calls(&self, option_enabled: bool) -> bool {
        option_enabled && self.enabled_opts.contains(&Optimization::UseTypeCheckedInlining)
    }

    /// Whether devirtualize-invokes is enabled.
    pub fn devirtualize_invokes(&self, option_enabled: bool) -> bool {
        option_enabled && self.enabled_opts.contains(&Optimization::UseTypeCheckedInlining)
    }

    /// Whether exception probability is enabled.
    pub fn use_exception_probability(&self, option_enabled: bool) -> bool {
        option_enabled && self.enabled_opts.contains(&Optimization::UseExceptionProbability)
    }

    /// Whether loop limit checks are enabled.
    pub fn use_loop_limit_checks(&self, option_enabled: bool) -> bool {
        option_enabled && self.enabled_opts.contains(&Optimization::UseLoopLimitChecks)
    }

    /// Whether this is less optimistic than the other.
    pub fn less_optimistic_than(&self, other: &Self) -> bool {
        for opt in &[
            Optimization::RemoveNeverExecutedCode,
            Optimization::UseTypeCheckedInlining,
            Optimization::UseTypeCheckHints,
            Optimization::UseExceptionProbabilityForOperations,
            Optimization::UseExceptionProbability,
            Optimization::UseLoopLimitChecks,
        ] {
            if !self.enabled_opts.contains(opt) && other.enabled_opts.contains(opt) {
                return true;
            }
        }
        false
    }
}

impl std::fmt::Display for OptimisticOptimizations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.enabled_opts)
    }
}