// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2012, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.
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

/// 镜像 `jdk.vm.ci.meta.DeoptimizationAction`：触发某 deoptimization 时 runtime 应采取的动作。
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum DeoptimizationAction {
    /// 不失效机器码（如编译期即可判定不会改变的负数组长度）。
    None,
    /// 不失效机器码，但若该 deopt 触发过频则调度重编译。
    RecompileIfTooManyDeopts,
    /// 失效机器码并重置 profiling 信息。
    InvalidateReprofile,
    /// 失效机器码并立即调度重编译（如解析未解析符号）。
    InvalidateRecompile,
    /// 失效机器码并停止编译本编译单元最外层方法。
    InvalidateStopCompiling,
}

impl DeoptimizationAction {
    pub fn does_invalidate_compilation(self) -> bool {
        match self {
            DeoptimizationAction::None => false,
            _ => true,
        }
    }
}

/// 镜像 `jdk.vm.ci.meta.DeoptimizationReason`：deoptimization 触发原因枚举。
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum DeoptimizationReason {
    None,
    NullCheckException,
    BoundsCheckException,
    ClassCastException,
    ArrayStoreException,
    UnreachedCode,
    TypeCheckedInliningViolated,
    OptimizedTypeCheckViolated,
    NotCompiledExceptionHandler,
    Unresolved,
    JavaSubroutineMismatch,
    ArithmeticException,
    RuntimeConstraint,
    LoopLimitCheck,
    Aliasing,
    TransferToInterpreter,
}

impl DeoptimizationReason {
    /// 对应 Java `DeoptimizationReason.values()`：返回全部枚举常量的切片。
    pub fn all() -> &'static [DeoptimizationReason] {
        &[
            DeoptimizationReason::None,
            DeoptimizationReason::NullCheckException,
            DeoptimizationReason::BoundsCheckException,
            DeoptimizationReason::ClassCastException,
            DeoptimizationReason::ArrayStoreException,
            DeoptimizationReason::UnreachedCode,
            DeoptimizationReason::TypeCheckedInliningViolated,
            DeoptimizationReason::OptimizedTypeCheckViolated,
            DeoptimizationReason::NotCompiledExceptionHandler,
            DeoptimizationReason::Unresolved,
            DeoptimizationReason::JavaSubroutineMismatch,
            DeoptimizationReason::ArithmeticException,
            DeoptimizationReason::RuntimeConstraint,
            DeoptimizationReason::LoopLimitCheck,
            DeoptimizationReason::Aliasing,
            DeoptimizationReason::TransferToInterpreter,
        ]
    }

    /// 对应 Java `Enum.name()`：返回枚举常量名。
    pub fn name(self) -> &'static str {
        match self {
            DeoptimizationReason::None => "None",
            DeoptimizationReason::NullCheckException => "NullCheckException",
            DeoptimizationReason::BoundsCheckException => "BoundsCheckException",
            DeoptimizationReason::ClassCastException => "ClassCastException",
            DeoptimizationReason::ArrayStoreException => "ArrayStoreException",
            DeoptimizationReason::UnreachedCode => "UnreachedCode",
            DeoptimizationReason::TypeCheckedInliningViolated => "TypeCheckedInliningViolated",
            DeoptimizationReason::OptimizedTypeCheckViolated => "OptimizedTypeCheckViolated",
            DeoptimizationReason::NotCompiledExceptionHandler => "NotCompiledExceptionHandler",
            DeoptimizationReason::Unresolved => "Unresolved",
            DeoptimizationReason::JavaSubroutineMismatch => "JavaSubroutineMismatch",
            DeoptimizationReason::ArithmeticException => "ArithmeticException",
            DeoptimizationReason::RuntimeConstraint => "RuntimeConstraint",
            DeoptimizationReason::LoopLimitCheck => "LoopLimitCheck",
            DeoptimizationReason::Aliasing => "Aliasing",
            DeoptimizationReason::TransferToInterpreter => "TransferToInterpreter",
        }
    }
}

impl std::fmt::Display for DeoptimizationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
