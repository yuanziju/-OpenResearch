// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2019, 2024, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.graal.compiler.lir.asm`：汇编抽象层。
//!
//! 偏离记录：Java 子包 `lir.asm` 含汇编缓冲区和汇编构建器抽象。
//! Rust 侧提供 trait 定义。

/// 对应 `lir.asm.CompilationResultBuilder`：汇编结果构建器。
///
/// 抽象汇编输出接口，平台无关。
pub trait CompilationResultBuilder {
    /// 发射字节。
    fn emit_byte(&mut self, b: u8);
    /// 发射短整型。
    fn emit_short(&mut self, s: i16);
    /// 发射整型。
    fn emit_int(&mut self, i: i32);
    /// 发射长整型。
    fn emit_long(&mut self, l: i64);
    /// 发射字节数组。
    fn emit_bytes(&mut self, bytes: &[u8]);
    /// 获取当前偏移。
    fn get_offset(&self) -> usize;
}

/// 对应 `lir.asm.TargetMethodAssembler`：目标方法汇编器。
pub trait TargetMethodAssembler {
    /// 获取汇编结果。
    fn get_result(&self) -> &dyn CompilationResultBuilder;
    /// 获取可变汇编结果。
    fn get_result_mut(&mut self) -> &mut dyn CompilationResultBuilder;
}