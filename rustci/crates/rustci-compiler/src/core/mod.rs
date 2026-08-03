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
// Rust mirror of `jdk.graal.compiler.core` — module declarations for the 10
// root-level classes and the phases/ sub-package.

pub mod architecture_specific;
pub mod compilation_printer;
pub mod compilation_watch_dog;
pub mod compilation_wrapper;
pub mod compiler_thread;
pub mod compiler_thread_factory;
pub mod graal_compiler;
pub mod graal_compiler_options;
pub mod instrumentation;
pub mod lir_generation_phase;
pub mod phases;

pub use architecture_specific::ArchitectureSpecific;
pub use compilation_printer::CompilationPrinter;
pub use compilation_watch_dog::CompilationWatchDog;
pub use compilation_wrapper::{CompilationWrapper, ExceptionAction};
pub use compiler_thread::CompilerThread;
pub use compiler_thread_factory::CompilerThreadFactory;
pub use graal_compiler::GraalCompiler;
pub use graal_compiler_options::GraalCompilerOptions;
pub use instrumentation::Instrumentation;
pub use lir_generation_phase::LIRGenerationPhase;
