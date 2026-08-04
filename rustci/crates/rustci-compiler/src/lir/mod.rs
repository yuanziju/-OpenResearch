// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
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

//! Rust mirror of `jdk.graal.compiler.lir` — Low-Level IR module.
//!
//! Faithful 1:1 port of the Graal LIR (Low-Level Intermediate Representation)
//! layer. The LIR is positioned between HIR (nodes) and machine code. It
//! contains basic blocks, SSA variables, abstract instructions, and
//! sub-packages for assembly abstraction, data-flow analysis, LIR generation,
//! compilation phases, SSA utilities, and stack-slot allocation.

pub mod block_value;
pub mod composite_value;
pub mod constant_value;
pub mod label_ref;
pub mod lir;
pub mod lir_frame_state;
pub mod lir_insertion_buffer;
pub mod lir_instruction;
pub mod lir_introspection;
pub mod lir_kind;
pub mod lir_value_class;
pub mod lir_verifier;
pub mod standard_op;
pub mod value_procedure;
pub mod variable;

pub mod asm;
pub mod dfa;
pub mod gen;
pub mod phases;
pub mod ssa;
pub mod stackslotalloc;

pub use block_value::BlockValue;
pub use composite_value::CompositeValue;
pub use constant_value::ConstantValue;
pub use label_ref::LabelRef;
pub use lir::LIR;
pub use lir::LIRBlock;
pub use lir_frame_state::{
    BytecodePosition, DebugInfo, LIRFrameState, ReferenceMap, StateConsumer, StateProcedure,
    VirtualObject,
};
pub use lir_insertion_buffer::LIRInsertionBuffer;
pub use lir_instruction::LIRInstruction;
pub use lir_instruction::LIRInstructionBase;
pub use lir_introspection::{LIRIntrospection, LIRIntrospectionValues};
pub use lir_kind::{DerivedReferenceBase, LIRKind};
pub use lir_value_class::LIRValueClass;
pub use lir_verifier::LIRVerifier;
pub use standard_op::{
    AllocOp, BlockEndOp, BranchOp, JumpOp, LabelOp, LoadConstantOp, MoveOp, NullCheck,
    RestoreRegistersOp, SaveRegistersOp,
};
pub use value_procedure::{
    InstructionStateProcedure, InstructionValueConsumer, InstructionValueProcedure,
    OperandFlag, OperandFlags, OperandMode, ValueConsumer, ValueProcedure,
};
pub use variable::Variable;