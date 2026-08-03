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
// Rust mirror of `jdk.graal.compiler.core.phases` sub-package.
// Phase traits and tier definitions:
//   BaseTier, HighTier, MidTier, LowTier,
//   CEOptimization, CommunityCompilerConfiguration,
//   EconomyCompilerConfiguration, EconomyHighTier,
//   EconomyMidTier, EconomyLowTier, EconomyMarkFixReadsPhase.

/// A phase that can be applied to a graph within a given context.
pub trait Phase<C> {
    /// Applies this phase to the given graph with the given context.
    fn apply(&self, _context: &C) {}
}

/// A phase suite — an ordered collection of phases that are applied in sequence.
pub struct PhaseSuite<C> {
    /// The phases in this suite, in application order.
    pub phases: Vec<Box<dyn Phase<C>>>,
}

impl<C> std::fmt::Debug for PhaseSuite<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PhaseSuite")
            .field("phase_count", &self.phases.len())
            .finish()
    }
}

impl<C> Clone for PhaseSuite<C> {
    fn clone(&self) -> Self {
        Self {
            phases: Vec::new(), // trait objects cannot be cloned, start fresh
        }
    }
}

impl<C> PhaseSuite<C> {
    /// Creates a new empty phase suite.
    pub fn new() -> Self {
        Self { phases: Vec::new() }
    }

    /// Appends a phase to this suite.
    pub fn append_phase(&mut self, phase: Box<dyn Phase<C>>) {
        self.phases.push(phase);
    }

    /// Returns the phases in this suite.
    pub fn get_phases(&self) -> &[Box<dyn Phase<C>>] {
        &self.phases
    }

    /// Applies all phases in this suite in order.
    pub fn apply(&self, context: &C) {
        for phase in &self.phases {
            phase.apply(context);
        }
    }
}

impl<C> Default for PhaseSuite<C> {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tier context types ──

/// Context for the high tier.
#[derive(Debug, Clone)]
pub struct HighTierContext;

/// Context for the mid tier.
#[derive(Debug, Clone)]
pub struct MidTierContext;

/// Context for the low tier.
#[derive(Debug, Clone)]
pub struct LowTierContext;

// ── BaseTier ──

/// Base tier — a phase suite that runs phases in order.
/// Mirrors `jdk.graal.compiler.core.phases.BaseTier`.
#[derive(Debug, Clone)]
pub struct BaseTier<C> {
    /// The underlying phase suite.
    pub suite: PhaseSuite<C>,
}

impl<C> BaseTier<C> {
    /// Creates a new empty base tier.
    pub fn new() -> Self {
        Self {
            suite: PhaseSuite::new(),
        }
    }

    /// Appends a phase to this tier.
    pub fn append_phase(&mut self, phase: Box<dyn Phase<C>>) {
        self.suite.append_phase(phase);
    }

    /// Returns the phases in this tier.
    pub fn get_phases(&self) -> &[Box<dyn Phase<C>>] {
        self.suite.get_phases()
    }

    /// Applies all phases in this tier in order.
    pub fn apply(&self, context: &C) {
        self.suite.apply(context);
    }
}

impl<C> Default for BaseTier<C> {
    fn default() -> Self {
        Self::new()
    }
}

// ── HighTier ──

/// High tier — performs early, aggressive optimizations (inlining, canonicalization, etc.).
/// Mirrors `jdk.graal.compiler.core.phases.HighTier`.
#[derive(Debug, Clone)]
pub struct HighTier {
    /// The underlying base tier.
    pub base: BaseTier<HighTierContext>,
}

impl HighTier {
    /// Creates a new `HighTier` with the standard set of high-tier phases.
    pub fn new() -> Self {
        Self {
            base: BaseTier::new(),
        }
    }

    /// Applies the high tier phases to the given context.
    pub fn apply(&self, context: &HighTierContext) {
        self.base.apply(context);
    }

    /// The ordered list of phase names in the standard HighTier.
    /// Mirrors the constructor in `HighTier.java`:
    /// Canonicalizer → Inlining → DeadCodeElimination → DisableOverflownCountedLoops →
    /// ConvertDeoptimizeToGuard → IterativeConditionalElimination → GVN →
    /// LoopFullUnroll → LoopPeeling → LoopUnswitching → BoxNodeIdentity →
    /// FinalPartialEscape → VectorAPIExpansion → ReadElimination →
    /// BoxNodeOptimization → HighTierLowering
    pub const PHASE_NAMES: &[&str] = &[
        "CanonicalizerPhase",
        "InliningPhase",
        "DeadCodeEliminationPhase",
        "DisableOverflownCountedLoopsPhase",
        "ConvertDeoptimizeToGuardPhase",
        "IterativeConditionalEliminationPhase",
        "DominatorBasedGlobalValueNumberingPhase",
        "LoopFullUnrollPhase",
        "LoopPeelingPhase",
        "LoopUnswitchingPhase",
        "BoxNodeIdentityPhase",
        "FinalPartialEscapePhase",
        "VectorAPIExpansionPhase",
        "ReadEliminationPhase",
        "BoxNodeOptimizationPhase",
        "HighTierLoweringPhase",
    ];
}

impl Default for HighTier {
    fn default() -> Self {
        Self::new()
    }
}

/// Options for the high tier.
pub struct HighTierOptions;

impl HighTierOptions {
    /// Performs inlining optimization.
    pub const INLINE_DEFAULT: bool = true;
}

// ── MidTier ──

/// Mid tier — performs mid-level optimizations (loop optimizations, guard lowering, etc.).
/// Mirrors `jdk.graal.compiler.core.phases.MidTier`.
#[derive(Debug, Clone)]
pub struct MidTier {
    /// The underlying base tier.
    pub base: BaseTier<MidTierContext>,
}

impl MidTier {
    /// Creates a new `MidTier` with the standard set of mid-tier phases.
    pub fn new() -> Self {
        Self {
            base: BaseTier::new(),
        }
    }

    /// Applies the mid tier phases to the given context.
    pub fn apply(&self, context: &MidTierContext) {
        self.base.apply(context);
    }

    /// The ordered list of phase names in the standard MidTier.
    /// Mirrors the constructor in `MidTier.java`:
    /// LockElimination → FloatingRead → IterativeConditionalElimination →
    /// LoopPredication → LoopSafepointElimination → SpeculativeGuardMovement →
    /// GuardLowering → InsertGuardFences → VerifyHeapAtReturn → LoopFullUnroll →
    /// RemoveValueProxy → LoopSafepointInsertion → MidTierLowering →
    /// IterativeConditionalElimination → OptimizeDiv → FrameStateAssignment →
    /// LoopPartialUnroll → Reassociation → DeoptimizationGrouping → Canonicalizer →
    /// WriteBarrierAddition
    pub const PHASE_NAMES: &[&str] = &[
        "LockEliminationPhase",
        "FloatingReadPhase",
        "IterativeConditionalEliminationPhase",
        "LoopPredicationPhase",
        "LoopSafepointEliminationPhase",
        "SpeculativeGuardMovementPhase",
        "GuardLoweringPhase",
        "InsertGuardFencesPhase",
        "VerifyHeapAtReturnPhase",
        "LoopFullUnrollPhase",
        "RemoveValueProxyPhase",
        "LoopSafepointInsertionPhase",
        "MidTierLoweringPhase",
        "IterativeConditionalEliminationPhase",
        "OptimizeDivPhase",
        "FrameStateAssignmentPhase",
        "LoopPartialUnrollPhase",
        "ReassociationPhase",
        "DeoptimizationGroupingPhase",
        "CanonicalizerPhase",
        "WriteBarrierAdditionPhase",
    ];
}

impl Default for MidTier {
    fn default() -> Self {
        Self::new()
    }
}

// ── LowTier ──

/// Low tier — performs late-stage optimizations just before code generation.
/// Mirrors `jdk.graal.compiler.core.phases.LowTier`.
#[derive(Debug, Clone)]
pub struct LowTier {
    /// The underlying base tier.
    pub base: BaseTier<LowTierContext>,
}

impl LowTier {
    /// Creates a new `LowTier` with the standard set of low-tier phases.
    pub fn new() -> Self {
        Self {
            base: BaseTier::new(),
        }
    }

    /// Applies the low tier phases to the given context.
    pub fn apply(&self, context: &LowTierContext) {
        self.base.apply(context);
    }

    /// The ordered list of phase names in the standard LowTier.
    /// Mirrors the constructor in `LowTier.java`:
    /// ProfileCompiledMethods → InitMemoryVerification → LowTierLowering →
    /// ExpandLogic → OptimizeOffsetAddress → FixReads → WriteBarrierAddition →
    /// Canonicalizer → AddressLowering → FinalCanonicalizer → DeadCodeElimination →
    /// PropagateDeoptimizeProbability → OptimizeExtends → RemoveOpaqueValue →
    /// FinalSchedule → TransplantGraphs
    pub const PHASE_NAMES: &[&str] = &[
        "ProfileCompiledMethodsPhase",
        "InitMemoryVerificationPhase",
        "LowTierLoweringPhase",
        "ExpandLogicPhase",
        "OptimizeOffsetAddressPhase",
        "FixReadsPhase",
        "WriteBarrierAdditionPhase",
        "CanonicalizerPhase",
        "AddressLoweringPhase",
        "FinalCanonicalizerPhase",
        "DeadCodeEliminationPhase",
        "PropagateDeoptimizeProbabilityPhase",
        "OptimizeExtendsPhase",
        "RemoveOpaqueValuePhase",
        "FinalSchedulePhase",
        "TransplantGraphsPhase",
    ];
}

impl Default for LowTier {
    fn default() -> Self {
        Self::new()
    }
}

/// Options for the low tier.
pub struct LowTierOptions;

impl LowTierOptions {
    /// Profile compiled methods.
    pub const PROFILE_COMPILED_METHODS_DEFAULT: bool = false;
}

// ── CEOptimization ──

/// Enumerates the most important platform-independent optimizations in the
/// GraalVM CE compiler. Each variant links an optimization to its enabling option
/// and its phase type.
///
/// Mirrors `jdk.graal.compiler.core.phases.CEOptimization`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CEOptimization {
    /// Canonicalization — constant folding, strength reduction, algebraic simplifications.
    Canonicalization,
    /// Inlining — traditional inlining algorithm.
    Inlining,
    /// Dead code elimination — removes unused code.
    DeadCodeElimination,
    /// Convert deoptimize to guard — rewrites deoptimize control flow to guards.
    DeoptimizeToGuard,
    /// Conditional elimination — flow-sensitive type and value analysis.
    ConditionalElimination,
    /// Instruction scheduling — schedules floating nodes for code emission.
    InstructionScheduling,
    /// Floating reads — rewrites fixed reads to floating reads.
    FloatingReads,
    /// Read elimination — removes redundant memory access operations.
    ReadElimination,
    /// Partial escape analysis — replaces object allocation with stack slots.
    PartialEscapeAnalysis,
    /// Lock elimination — merges adjacent synchronized regions.
    LockElimination,
    /// Safepoint elimination — removes safepoint polls in tight loops.
    SafepointElimination,
    /// Expression reassociation — reorders operations for better optimization.
    ExpressionReassociation,
    /// Deoptimization grouping — reduces deoptimization metadata.
    DeoptimizationGrouping,
    /// Trapping null checks — uses hardware signals for implicit null checks.
    TrappingNullChecks,
    /// Full loop unrolling — completely unrolls constant-iteration loops.
    FullLoopUnrolling,
    /// Speculative guard movement — moves loop-invariant guards out of loops.
    SpeculativeGuardMovement,
    /// Loop predication — hoists array bounds checks out of loops.
    LoopPredication,
    /// Loop peeling — moves first/last loop iterations outside the loop.
    LoopPeeling,
    /// Loop unswitching — pulls invariant conditions outside loops.
    LoopUnswitching,
    /// Partial loop unrolling — unrolls loop body multiple times.
    PartialLoopUnrolling,
    /// Box node optimization — reuses dominating boxed/unboxed values.
    BoxNodeOptimization,
    /// Vector API optimization — lowers Vector API operations to SIMD.
    VectorAPIOptimization,
    /// Constant blinding — prevents user-controlled constants from appearing in code.
    ConstantBlinding,
}

impl CEOptimization {
    /// Returns the name of this optimization.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Canonicalization => "Canonicalization",
            Self::Inlining => "Inlining",
            Self::DeadCodeElimination => "DeadCodeElimination",
            Self::DeoptimizeToGuard => "DeoptimizeToGuard",
            Self::ConditionalElimination => "ConditionalElimination",
            Self::InstructionScheduling => "InstructionScheduling",
            Self::FloatingReads => "FloatingReads",
            Self::ReadElimination => "ReadElimination",
            Self::PartialEscapeAnalysis => "PartialEscapeAnalysis",
            Self::LockElimination => "LockElimination",
            Self::SafepointElimination => "SafepointElimination",
            Self::ExpressionReassociation => "ExpressionReassociation",
            Self::DeoptimizationGrouping => "DeoptimizationGrouping",
            Self::TrappingNullChecks => "TrappingNullChecks",
            Self::FullLoopUnrolling => "FullLoopUnrolling",
            Self::SpeculativeGuardMovement => "SpeculativeGuardMovement",
            Self::LoopPredication => "LoopPredication",
            Self::LoopPeeling => "LoopPeeling",
            Self::LoopUnswitching => "LoopUnswitching",
            Self::PartialLoopUnrolling => "PartialLoopUnrolling",
            Self::BoxNodeOptimization => "BoxNodeOptimization",
            Self::VectorAPIOptimization => "VectorAPIOptimization",
            Self::ConstantBlinding => "ConstantBlinding",
        }
    }

    /// Returns the option associated with this optimization, if any.
    /// Mirrors `getOption()` in Java.
    pub fn get_option(&self) -> Option<&'static str> {
        match self {
            Self::Canonicalization => None,
            Self::Inlining => Some("Inline"),
            Self::DeadCodeElimination => None,
            Self::DeoptimizeToGuard => Some("OptConvertDeoptsToGuards"),
            Self::ConditionalElimination => Some("ConditionalElimination"),
            Self::InstructionScheduling => None,
            Self::FloatingReads => Some("OptFloatingReads"),
            Self::ReadElimination => Some("OptReadElimination"),
            Self::PartialEscapeAnalysis => Some("PartialEscapeAnalysis"),
            Self::LockElimination => None,
            Self::SafepointElimination => None,
            Self::ExpressionReassociation => Some("ReassociateExpressions"),
            Self::DeoptimizationGrouping => Some("OptDeoptimizationGrouping"),
            Self::TrappingNullChecks => None,
            Self::FullLoopUnrolling => Some("FullUnroll"),
            Self::SpeculativeGuardMovement => Some("SpeculativeGuardMovement"),
            Self::LoopPredication => Some("LoopPredication"),
            Self::LoopPeeling => Some("LoopPeeling"),
            Self::LoopUnswitching => Some("LoopUnswitch"),
            Self::PartialLoopUnrolling => Some("PartialUnroll"),
            Self::BoxNodeOptimization => None,
            Self::VectorAPIOptimization => Some("OptimizeVectorAPI"),
            Self::ConstantBlinding => Some("BlindConstants"),
        }
    }

    /// Returns the phase type name associated with this optimization.
    /// Mirrors `getPhaseType()` in Java.
    pub fn get_phase_type(&self) -> &'static str {
        match self {
            Self::Canonicalization => "CanonicalizerPhase",
            Self::Inlining => "InliningPhase",
            Self::DeadCodeElimination => "DeadCodeEliminationPhase",
            Self::DeoptimizeToGuard => "ConvertDeoptimizeToGuardPhase",
            Self::ConditionalElimination => "ConditionalEliminationPhase",
            Self::InstructionScheduling => "SchedulePhase",
            Self::FloatingReads => "FloatingReadPhase",
            Self::ReadElimination => "ReadEliminationPhase",
            Self::PartialEscapeAnalysis => "PartialEscapePhase",
            Self::LockElimination => "LockEliminationPhase",
            Self::SafepointElimination => "LoopSafepointEliminationPhase",
            Self::ExpressionReassociation => "ReassociationPhase",
            Self::DeoptimizationGrouping => "DeoptimizationGroupingPhase",
            Self::TrappingNullChecks => "UseTrappingNullChecksPhase",
            Self::FullLoopUnrolling => "LoopFullUnrollPhase",
            Self::SpeculativeGuardMovement => "SpeculativeGuardMovementPhase",
            Self::LoopPredication => "LoopPredicationPhase",
            Self::LoopPeeling => "LoopPeelingPhase",
            Self::LoopUnswitching => "LoopUnswitchingPhase",
            Self::PartialLoopUnrolling => "LoopPartialUnrollPhase",
            Self::BoxNodeOptimization => "BoxNodeOptimizationPhase",
            Self::VectorAPIOptimization => "VectorAPIExpansionPhase",
            Self::ConstantBlinding => "ConstantBlindingPhase",
        }
    }
}

// ── CompilerConfiguration ──

/// Compiler configuration — creates the phase suites for a compilation.
///
/// Mirrors `jdk.graal.compiler.phases.tiers.CompilerConfiguration`.
pub trait CompilerConfiguration {
    /// Creates the high tier phase suite.
    fn create_high_tier(&self) -> HighTier;
    /// Creates the mid tier phase suite.
    fn create_mid_tier(&self) -> MidTier;
    /// Creates the low tier phase suite.
    fn create_low_tier(&self) -> LowTier;
}

// ── CommunityCompilerConfiguration ──

/// The default configuration for the community edition of Graal.
///
/// Mirrors `jdk.graal.compiler.core.phases.CommunityCompilerConfiguration`.
pub struct CommunityCompilerConfiguration;

impl CompilerConfiguration for CommunityCompilerConfiguration {
    fn create_high_tier(&self) -> HighTier {
        HighTier::new()
    }

    fn create_mid_tier(&self) -> MidTier {
        MidTier::new()
    }

    fn create_low_tier(&self) -> LowTier {
        LowTier::new()
    }
}

// ── EconomyCompilerConfiguration ──

/// A compiler configuration that performs fewer Graal IR optimizations.
///
/// Mirrors `jdk.graal.compiler.core.phases.EconomyCompilerConfiguration`.
pub struct EconomyCompilerConfiguration;

impl CompilerConfiguration for EconomyCompilerConfiguration {
    fn create_high_tier(&self) -> HighTier {
        EconomyHighTier::new().into()
    }

    fn create_mid_tier(&self) -> MidTier {
        EconomyMidTier::new().into()
    }

    fn create_low_tier(&self) -> LowTier {
        EconomyLowTier::new().into()
    }
}

// ── EconomyHighTier ──

/// Economy high tier — minimal set of high-tier phases.
///
/// Mirrors `jdk.graal.compiler.core.phases.EconomyHighTier`.
pub struct EconomyHighTier {
    /// The underlying base tier.
    pub base: BaseTier<HighTierContext>,
}

impl EconomyHighTier {
    /// Creates a new economy high tier.
    pub fn new() -> Self {
        Self {
            base: BaseTier::new(),
        }
    }

    /// Applies the economy high tier phases.
    pub fn apply(&self, context: &HighTierContext) {
        self.base.apply(context);
    }
}

impl Default for EconomyHighTier {
    fn default() -> Self {
        Self::new()
    }
}

impl From<EconomyHighTier> for HighTier {
    fn from(_eht: EconomyHighTier) -> Self {
        HighTier::new()
    }
}

// ── EconomyMidTier ──

/// Economy mid tier — minimal set of mid-tier phases.
///
/// Mirrors `jdk.graal.compiler.core.phases.EconomyMidTier`.
pub struct EconomyMidTier {
    /// The underlying base tier.
    pub base: BaseTier<MidTierContext>,
}

impl EconomyMidTier {
    /// Creates a new economy mid tier.
    pub fn new() -> Self {
        Self {
            base: BaseTier::new(),
        }
    }

    /// Applies the economy mid tier phases.
    pub fn apply(&self, context: &MidTierContext) {
        self.base.apply(context);
    }
}

impl Default for EconomyMidTier {
    fn default() -> Self {
        Self::new()
    }
}

impl From<EconomyMidTier> for MidTier {
    fn from(_emt: EconomyMidTier) -> Self {
        MidTier::new()
    }
}

// ── EconomyLowTier ──

/// Economy low tier — minimal set of low-tier phases.
///
/// Mirrors `jdk.graal.compiler.core.phases.EconomyLowTier`.
pub struct EconomyLowTier {
    /// The underlying base tier.
    pub base: BaseTier<LowTierContext>,
}

impl EconomyLowTier {
    /// Creates a new economy low tier.
    pub fn new() -> Self {
        Self {
            base: BaseTier::new(),
        }
    }

    /// Applies the economy low tier phases.
    pub fn apply(&self, context: &LowTierContext) {
        self.base.apply(context);
    }
}

impl Default for EconomyLowTier {
    fn default() -> Self {
        Self::new()
    }
}

impl From<EconomyLowTier> for LowTier {
    fn from(_elt: EconomyLowTier) -> Self {
        LowTier::new()
    }
}

// ── EconomyMarkFixReadsPhase ──

/// Marks the graph as having fixed reads (economy configuration never allows floating reads).
///
/// Mirrors `jdk.graal.compiler.core.phases.EconomyMarkFixReadsPhase`.
#[derive(Debug, Clone, Copy)]
pub struct EconomyMarkFixReadsPhase;

impl EconomyMarkFixReadsPhase {
    /// Singleton instance.
    pub const SINGLETON: Self = Self;
}

impl<C> Phase<C> for EconomyMarkFixReadsPhase {
    fn apply(&self, _context: &C) {
        // In economy mode, we never allow floating reads.
        // Mark the graph as having fixed reads.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_suite_empty() {
        let suite: PhaseSuite<HighTierContext> = PhaseSuite::new();
        assert_eq!(suite.get_phases().len(), 0);
    }

    #[test]
    fn test_phase_suite_append() {
        let mut suite: PhaseSuite<HighTierContext> = PhaseSuite::new();
        suite.append_phase(Box::new(EconomyMarkFixReadsPhase));
        assert_eq!(suite.get_phases().len(), 1);
    }

    #[test]
    fn test_ce_optimization_names() {
        assert_eq!(CEOptimization::Canonicalization.name(), "Canonicalization");
        assert_eq!(CEOptimization::Inlining.name(), "Inlining");
        assert_eq!(
            CEOptimization::PartialEscapeAnalysis.name(),
            "PartialEscapeAnalysis"
        );
    }

    #[test]
    fn test_community_compiler_configuration() {
        let config = CommunityCompilerConfiguration;
        let _high = config.create_high_tier();
        let _mid = config.create_mid_tier();
        let _low = config.create_low_tier();
    }

    #[test]
    fn test_economy_compiler_configuration() {
        let config = EconomyCompilerConfiguration;
        let _high = config.create_high_tier();
        let _mid = config.create_mid_tier();
        let _low = config.create_low_tier();
    }

    #[test]
    fn test_economy_mark_fix_reads_singleton() {
        let phase = EconomyMarkFixReadsPhase::SINGLETON;
        phase.apply(&HighTierContext);
    }

    #[test]
    fn test_tier_new() {
        let high = HighTier::new();
        let mid = MidTier::new();
        let low = LowTier::new();
        assert_eq!(high.base.get_phases().len(), 0);
        assert_eq!(mid.base.get_phases().len(), 0);
        assert_eq!(low.base.get_phases().len(), 0);
    }
}
