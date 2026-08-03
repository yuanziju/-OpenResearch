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
// Rust mirror of `jdk.graal.compiler.core.GraalCompiler`.
// Static methods for orchestrating the compilation of a graph.

use crate::core::compilation_printer::CompilationSource;
use crate::core::compilation_watch_dog::CompilationIdentifier;
use crate::core::phases::{HighTier, HighTierContext, LowTier, LowTierContext, MidTier, MidTierContext};

/// Result of a compilation — the compiled code and associated metadata.
#[derive(Debug, Clone)]
pub struct CompilationResult {
    /// The compilation identifier.
    pub id: CompilationIdentifier,
    /// The source being compiled.
    pub source: CompilationSource,
    /// The target code size in bytes, or -1 if unknown.
    pub target_code_size: i32,
    /// The bytecode size of the compiled method.
    pub bytecode_size: i32,
    /// The installed code start address, or 0 if not installed.
    pub start_address: u64,
}

impl CompilationResult {
    /// Target code size value for unknown/unavailable code size.
    pub const UNKNOWN_TARGET_CODE_SIZE: i32 = -1;

    /// Creates a new `CompilationResult`.
    pub fn new(id: CompilationIdentifier, source: CompilationSource) -> Self {
        Self {
            id,
            source,
            target_code_size: Self::UNKNOWN_TARGET_CODE_SIZE,
            bytecode_size: 0,
            start_address: 0,
        }
    }

    /// Returns the target code size.
    pub fn get_target_code_size(&self) -> i32 {
        self.target_code_size
    }

    /// Returns the bytecode size.
    pub fn get_bytecode_size(&self) -> i32 {
        self.bytecode_size
    }
}

/// Encapsulates all the inputs to a compilation.
///
/// Mirrors `GraalCompiler.Request<T>` record in Java, which has 14 fields:
/// graph, installedCodeOwner, providers, backend, graphBuilderSuite,
/// optimisticOpts, profilingInfo, suites, lirSuites, compilationResult,
/// factory, entryPointDecorator, requestedCrashHandler, verifySourcePositions.
#[derive(Debug)]
pub struct CompilationRequest {
    /// The compilation identifier.
    pub id: CompilationIdentifier,
    /// The source of the compilation.
    pub source: CompilationSource,
    /// The entry BCI, or -1 for invocation entry.
    pub entry_bci: i32,
    /// Whether to verify source positions.
    pub verify_source_positions: bool,
    /// The high tier phase suite.
    pub high_tier: HighTier,
    /// The mid tier phase suite.
    pub mid_tier: MidTier,
    /// The low tier phase suite.
    pub low_tier: LowTier,
    /// Whether to emit back-end (LIR + code gen).
    pub emit_backend: bool,
}

impl CompilationRequest {
    /// Creates a new `CompilationRequest`.
    pub fn new(
        id: CompilationIdentifier,
        source: CompilationSource,
        entry_bci: i32,
        verify_source_positions: bool,
    ) -> Self {
        Self {
            id,
            source,
            entry_bci,
            verify_source_positions,
            high_tier: HighTier::new(),
            mid_tier: MidTier::new(),
            low_tier: LowTier::new(),
            emit_backend: false,
        }
    }

    /// Creates a new `CompilationRequest` with pre-configured tiers and backend flag.
    pub fn with_tiers(
        id: CompilationIdentifier,
        source: CompilationSource,
        entry_bci: i32,
        verify_source_positions: bool,
        high_tier: HighTier,
        mid_tier: MidTier,
        low_tier: LowTier,
        emit_backend: bool,
    ) -> Self {
        Self {
            id,
            source,
            entry_bci,
            verify_source_positions,
            high_tier,
            mid_tier,
            low_tier,
            emit_backend,
        }
    }

    /// Executes this compilation request.
    pub fn execute(&self) -> CompilationResult {
        GraalCompiler::compile(self)
    }
}

/// Handler for requested crashes during compilation (triggered by CrashAt option).
pub trait RequestedCrashHandler {
    /// Returns true if the caller should proceed to throw an exception.
    fn notify_crash(&self, crash_message: &str) -> bool;
}

/// A default no-op crash handler.
pub struct DefaultCrashHandler;

impl RequestedCrashHandler for DefaultCrashHandler {
    fn notify_crash(&self, _crash_message: &str) -> bool {
        true
    }
}

/// Static methods for orchestrating the compilation of a graph.
pub struct GraalCompiler;

impl GraalCompiler {
    /// Compiler timer name.
    pub const COMPILER_TIMER_NAME: &'static str = "GraalCompiler";
    /// Front-end timer name.
    pub const FRONT_END_TIMER_NAME: &'static str = "FrontEnd";

    /// Services a given compilation request.
    ///
    /// Mirrors `GraalCompiler.compile(Request<T>)`:
    /// 1. Runs emitFrontEnd (graph building → high tier → mid tier → low tier)
    /// 2. Optionally emits back-end (LIR → code generation)
    /// 3. Checks for requested crash and delay
    /// 4. Returns the compilation result
    pub fn compile(request: &CompilationRequest) -> CompilationResult {
        // Execute front-end phases: parse → high tier → mid tier → low tier
        GraalCompiler::emit_front_end(request);

        // Optionally emit back-end
        if request.emit_backend {
            // In the full implementation, this would call backend.emitBackEnd(...)
            // with the graph, LIR suites, and compilation result builder factory.
        }

        // Check for requested crash
        // In the full implementation, this would check CrashAt option.
        // Check for requested delay
        // In the full implementation, this would check InjectedCompilationDelay option.

        let mut result = CompilationResult::new(request.id.clone(), request.source.clone());
        result.bytecode_size = 0; // Would be set by actual compilation
        result
    }

    /// Builds the graph and optimizes it (front-end phase).
    ///
    /// Mirrors `GraalCompiler.emitFrontEnd()`:
    /// 1. Graph building (parsing) via graphBuilderSuite
    /// 2. High tier optimizations
    /// 3. Mid tier optimizations
    /// 4. Low tier optimizations
    /// 5. Final HIR schedule dump
    pub fn emit_front_end(request: &CompilationRequest) {
        // Phase 1: Graph building / parsing
        // In the full implementation, this applies graphBuilderSuite to the graph.
        // The graphBuilderSuite contains bytecode parsing phases that build the
        // initial HIR from the method's bytecodes.
        let high_ctx = HighTierContext;
        // graphBuilderSuite.apply(graph, high_tier_context);

        // Phase 2: High tier — aggressive early optimizations
        // This includes canonicalization, inlining, dead code elimination,
        // conditional elimination, etc.
        request.high_tier.apply(&high_ctx);

        // Phase 3: Mid tier — loop optimizations, guard lowering, etc.
        let mid_ctx = MidTierContext;
        request.mid_tier.apply(&mid_ctx);

        // Phase 4: Low tier — late-stage optimizations before code generation
        // This includes address lowering, final canonicalization, etc.
        let low_ctx = LowTierContext;
        request.low_tier.apply(&low_ctx);

        // Final HIR schedule would be dumped here in the full implementation.
    }

    /// Checks whether the CrashAt option indicates that the compilation
    /// should result in an exception.
    pub fn check_for_requested_crash(
        crash_at_value: Option<&str>,
        method_name: &str,
        crash_handler: Option<&dyn RequestedCrashHandler>,
    ) -> Result<(), String> {
        let value = match crash_at_value {
            Some(v) => v,
            None => return Ok(()),
        };

        let mut bailout = false;
        let mut permanent_bailout = false;
        let method_pattern = if let Some(stripped) = value.strip_suffix(":Bailout") {
            bailout = true;
            stripped
        } else if let Some(stripped) = value.strip_suffix(":PermanentBailout") {
            permanent_bailout = true;
            stripped
        } else {
            value
        };

        if method_name.contains(method_pattern) {
            let crash_message = format!("Forced crash after compiling {}", method_name);
            let should_crash = crash_handler.is_none_or(|h| h.notify_crash(&crash_message));
            if should_crash {
                return Err(if permanent_bailout {
                    format!("PermanentBailout: {}", crash_message)
                } else if bailout {
                    format!("RetryableBailout: {}", crash_message)
                } else {
                    format!("RuntimeException: {}", crash_message)
                });
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile() {
        let id = CompilationIdentifier::new("1");
        let source = CompilationSource::java_method("Test", "foo", "()V");
        let request = CompilationRequest::new(id, source, -1, false);
        let result = request.execute();
        assert_eq!(
            result.get_target_code_size(),
            CompilationResult::UNKNOWN_TARGET_CODE_SIZE
        );
    }

    #[test]
    fn test_compile_with_tiers() {
        let id = CompilationIdentifier::new("2");
        let source = CompilationSource::java_method("Test", "bar", "(I)I");
        let request = CompilationRequest::with_tiers(
            id,
            source,
            -1,
            false,
            HighTier::new(),
            MidTier::new(),
            LowTier::new(),
            false,
        );
        let result = request.execute();
        assert_eq!(result.bytecode_size, 0);
    }

    #[test]
    fn test_crash_at_no_match() {
        let result = GraalCompiler::check_for_requested_crash(Some("Foo.bar"), "Test.foo", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_crash_at_match() {
        let result = GraalCompiler::check_for_requested_crash(Some("Test.foo"), "Test.foo", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_crash_at_bailout() {
        let result =
            GraalCompiler::check_for_requested_crash(Some("Test.foo:Bailout"), "Test.foo", None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("RetryableBailout"));
    }

    #[test]
    fn test_crash_at_permanent_bailout() {
        let result = GraalCompiler::check_for_requested_crash(
            Some("Test.foo:PermanentBailout"),
            "Test.foo",
            None,
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("PermanentBailout"));
    }

    #[test]
    fn test_crash_at_none() {
        let result = GraalCompiler::check_for_requested_crash(None, "Test.foo", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compilation_result() {
        let id = CompilationIdentifier::new("42");
        let source = CompilationSource::java_method("Test", "bar", "(I)I");
        let result = CompilationResult::new(id, source);
        assert_eq!(result.target_code_size, -1);
        assert_eq!(result.bytecode_size, 0);
    }
}