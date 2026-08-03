// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2010, 2025, Oracle and/or its affiliates. All rights reserved.
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
 * or visit oracle.com if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.code.CodeUtil`：`jdk.vm.ci.code` 及其客户端使用的工具方法集合（全静态）。
//!
//! 偏离记录：
//! - Java `class CodeUtil`（全静态方法 + 嵌套 `RefMapFormatter`/`DefaultRefMapFormatter`/
//!   `NumberedRefMapFormatter`）→ Rust 模块级 `pub fn` + 同文件 `pub trait RefMapFormatter`
//!   + `pub struct DefaultRefMapFormatter`/`NumberedRefMapFormatter`。
//! - `StringBuilder` → `&mut String`（用 `std::fmt::Write` 写入）。`System.lineSeparator()`
//!   → `"\n"`（Unix；JVMCI 测试基准为 Unix）。
//! - `String.format(Locale.ENGLISH, ...)` → `format!`。
//! - `tabulateValues`/`tabulate` 的 `Object[]` cells → `Vec<String>`（预格式化）。Java
//!   `String.valueOf(Object)` 对 cell 字符串化（null→"null"，Integer→数字，JavaValue→toString）；
//!   Rust 侧 frame `values` 元素为 `Box<dyn JavaValue>`，`JavaValue` 仅 `Debug`（无 `Display`），
//!   故 cell 用 `format!("{:?}", v)`（偏离 Java `toString`，但 `tabulateValues` 仅由
//!   `append_frame` 在帧含值时调用，本 crate 无测试触发该路径）。
//! - `tabulate` 列宽：Java `String.valueOf(cell).length()` 按 UTF-16 char 计；Rust 按 byte 计
//!   （`String::len`）。寄存器名/数字均为 ASCII，行为一致。
//! - `getCallingConvention`：Java `JavaType[] argTypes` 持引用；Rust 侧 `Signature::
//!   get_parameter_type` 返回 owned `Box<dyn JavaType>`，故先收集到本地 `Vec<Box<dyn JavaType>>`
//!   再取引用构造 `Vec<&dyn JavaType>`（对齐 `RegisterConfig::get_calling_convention` 签名）。
//!   `method.getDeclaringClass()` 返回 `&dyn JavaType`，直接入列。

use std::fmt::Write;

use crate::code::bytecode_frame::BytecodeFrame;
use crate::code::bytecode_position::BytecodePosition;
use crate::code::calling_convention::{CallingConvention, Type as CallingConventionType};
use crate::code::code_cache_provider::CodeCacheProvider;
use crate::code::debug_info::DebugInfo;
use crate::code::register::Register;
use crate::code::value_kind_factory::ValueKindFactory;
use crate::meta::java_type::JavaType;
use crate::meta::meta_util;
use crate::meta::resolved_java_method::ResolvedJavaMethod;

/// 对应 `public static final String NEW_LINE = System.lineSeparator()`。
pub const NEW_LINE: &str = "\n";

/// 对应 `public static final int K = 1024`。
pub const K: i32 = 1024;

/// 对应 `public static final int M = 1024 * 1024`。
pub const M: i32 = 1024 * 1024;

/// 对应 `static boolean isOdd(int n)`。
pub fn is_odd(n: i32) -> bool {
    (n & 1) == 1
}

/// 对应 `static boolean isEven(int n)`。
pub fn is_even(n: i32) -> bool {
    (n & 1) == 0
}

/// 对应 `static boolean isPowerOf2(int val)`。
pub fn is_power_of2_i32(val: i32) -> bool {
    val > 0 && (val & (val - 1)) == 0
}

/// 对应 `static boolean isPowerOf2(long val)`。
pub fn is_power_of2_i64(val: i64) -> bool {
    val > 0 && (val & (val - 1)) == 0
}

/// 对应 `static int log2(int val)`（`Integer.SIZE - 1 - numberOfLeadingZeros`）。
pub fn log2_i32(val: i32) -> i32 {
    assert!(val > 0);
    (i32::BITS as i32 - 1) - (val as u32).leading_zeros() as i32
}

/// 对应 `static int log2(long val)`（`Long.SIZE - 1 - numberOfLeadingZeros`）。
pub fn log2_i64(val: i64) -> i32 {
    assert!(val > 0);
    (u64::BITS as i32 - 1) - (val as u64).leading_zeros() as i32
}

/// 对应 `static long narrow(long value, int resultBits)`。
pub fn narrow(value: i64, result_bits: i32) -> i64 {
    let ret = value & mask(result_bits);
    sign_extend(ret, result_bits)
}

/// 对应 `static long signExtend(long value, int inputBits)`。
pub fn sign_extend(value: i64, input_bits: i32) -> i64 {
    if input_bits < 64 {
        debug_assert!(input_bits >= 0);
        // Java `value >>> (inputBits - 1)`：移位量按 mod 64 取（Java 语义），用 `& 63` 复刻，
        // 使 `inputBits == 0` 时退化为 `>>> 63`（与 Java 一致）。
        let shift = (input_bits.wrapping_sub(1) & 63) as u32;
        if ((value as u64) >> shift) & 1 == 1 {
            value | (-1i64 << (input_bits as u32 & 63))
        } else {
            value & !(-1i64 << (input_bits as u32 & 63))
        }
    } else {
        value
    }
}

/// 对应 `static long zeroExtend(long value, int inputBits)`。
pub fn zero_extend(value: i64, input_bits: i32) -> i64 {
    if input_bits < 64 {
        value & !(-1i64 << (input_bits as u32 & 63))
    } else {
        value
    }
}

/// 对应 `static long convert(long value, int inputBits, boolean unsigned)`。
pub fn convert(value: i64, input_bits: i32, unsigned: bool) -> i64 {
    if unsigned {
        zero_extend(value, input_bits)
    } else {
        sign_extend(value, input_bits)
    }
}

/// 对应 `static long mask(int bits)`。
pub fn mask(bits: i32) -> i64 {
    assert!((0..=64).contains(&bits));
    if bits == 64 {
        -1i64
    } else {
        (1i64 << bits) - 1
    }
}

/// 对应 `static long minValue(int bits)`。
pub fn min_value(bits: i32) -> i64 {
    assert!(0 < bits && bits <= 64);
    -1i64 << (bits - 1)
}

/// 对应 `static long maxValue(int bits)`。
pub fn max_value(bits: i32) -> i64 {
    assert!(0 < bits && bits <= 64);
    mask(bits - 1)
}

/// 对应 `static String tabulateValues(BytecodeFrame frame)`。
///
/// 偏离：参数为 `&BytecodePosition`（帧数据嵌入 `BytecodePosition`，见 `bytecode_frame.rs` 偏离）；
/// `pos` 须含帧数据。cell 值用 `{:?}` 格式化（见模块偏离记录）。
pub fn tabulate_values(pos: &BytecodePosition) -> String {
    let f = pos.frame_data().expect("frame data");
    let num_locals = f.num_locals as usize;
    let num_stack = f.num_stack as usize;
    let num_locks = f.num_locks as usize;
    let cols = num_locals.max(num_stack).max(num_locks);
    assert!(cols > 0);
    let mut cells: Vec<String> = Vec::new();
    cells.push(String::new());
    for i in 0..cols {
        cells.push(format!("{}", i));
    }
    let cols = cols + 1;
    if num_locals != 0 {
        cells.push("locals:".to_string());
        for i in 0..num_locals {
            cells.push(format!("{:?}", &*f.values[i]));
        }
        for _ in 0..(cols - num_locals - 1) {
            cells.push(String::new());
        }
    }
    if num_stack != 0 {
        cells.push("stack:".to_string());
        for i in 0..num_stack {
            cells.push(format!("{:?}", &*f.values[num_locals + i]));
        }
        for _ in 0..(cols - num_stack - 1) {
            cells.push(String::new());
        }
    }
    if num_locks != 0 {
        cells.push("locks:".to_string());
        for i in 0..num_locks {
            cells.push(format!("{:?}", &*f.values[num_locals + num_stack + i]));
        }
        for _ in 0..(cols - num_locks - 1) {
            cells.push(String::new());
        }
    }
    for (i, cell) in cells.iter_mut().enumerate() {
        if i % cols != 0 {
            *cell = format!("|{}", cell);
        }
    }
    tabulate(&cells, cols, 1, 1)
}

/// 对应 `static String tabulate(Object[] cells, int cols, int lpad, int rpad)`。
pub fn tabulate(cells: &[String], cols: usize, lpad: usize, rpad: usize) -> String {
    let rows = cells.len().div_ceil(cols);
    let mut col_widths = vec![0usize; cols];
    for (col, col_width) in col_widths.iter_mut().enumerate().take(cols) {
        for row in 0..rows {
            let index = col + row * cols;
            if index < cells.len() {
                *col_width = (*col_width).max(cells[index].len());
            }
        }
    }
    let mut sb = String::new();
    for row in 0..rows {
        for (col, col_width) in col_widths.iter().enumerate().take(cols) {
            let index = col + row * cols;
            if index < cells.len() {
                for _ in 0..lpad {
                    sb.push(' ');
                }
                let s = &cells[index];
                sb.push_str(s);
                let mut w = s.len();
                while w < *col_width {
                    sb.push(' ');
                    w += 1;
                }
                for _ in 0..rpad {
                    sb.push(' ');
                }
            }
        }
        sb.push_str(NEW_LINE);
    }
    sb
}

/// 对应 `static StringBuilder append(StringBuilder sb, BytecodePosition pos)`。
pub fn append_position(buf: &mut String, pos: &BytecodePosition) {
    buf.push_str("at ");
    meta_util::append_location(buf, Some(pos.get_method()), pos.get_bci());
    if let Some(caller) = pos.get_caller() {
        buf.push_str(NEW_LINE);
        append_position(buf, caller);
    }
}

/// 对应 `static StringBuilder append(StringBuilder sb, BytecodeFrame frame)`。
pub fn append_frame(buf: &mut String, frame: &BytecodeFrame) {
    append_frame_inner(buf, frame.as_position());
}

/// `append(BytecodeFrame)` 内部实现：`pos` 须含帧数据。复用于 `append_debug_info` 的帧路径
/// 与 caller 帧（caller 为 `&BytecodePosition`，无法构造 `&BytecodeFrame`，见 `bytecode_frame.rs` 偏离）。
fn append_frame_inner(buf: &mut String, pos: &BytecodePosition) {
    buf.push_str("at ");
    meta_util::append_location(buf, Some(pos.get_method()), pos.get_bci());
    // 对应 `assert sb.charAt(sb.length() - 1) == ']'` + `sb.deleteCharAt`：`appendLocation`
    // 末尾追加 ` [bci: N]`，弹出末尾 `]` 以拼接 `, duringCall: ..., rethrow: ...]`。
    buf.pop();
    let f = pos.frame_data().expect("frame data");
    let _ = write!(
        buf,
        ", duringCall: {}, rethrow: {}]",
        f.during_call, f.rethrow_exception
    );
    if !f.values.is_empty() {
        buf.push_str(NEW_LINE);
        let table = tabulate_values(pos);
        let rows: Vec<&str> = table.split(NEW_LINE).collect();
        let last = rows.len().saturating_sub(1);
        for (i, row) in rows.iter().enumerate() {
            if !row.trim().is_empty() {
                buf.push_str("  ");
                buf.push_str(row);
                if i != last {
                    buf.push_str(NEW_LINE);
                }
            }
        }
    }
    if let Some(caller) = pos.get_caller() {
        buf.push_str(NEW_LINE);
        if caller.has_frame() {
            append_frame_inner(buf, caller);
        } else {
            append_position(buf, caller);
        }
    }
}

/// 对应 `interface RefMapFormatter`。
pub trait RefMapFormatter {
    /// 对应 `formatStackSlot(int frameRefMapIndex)`。
    fn format_stack_slot(&self, frame_ref_map_index: i32) -> String;
}

/// 对应 `static class DefaultRefMapFormatter implements RefMapFormatter`。
pub struct DefaultRefMapFormatter {
    /// 对应 `public final int slotSize`。
    pub slot_size: i32,
    /// 对应 `public final Register fp`。
    pub fp: Register,
    /// 对应 `public final int refMapToFPOffset`。
    pub ref_map_to_fp_offset: i32,
}

impl DefaultRefMapFormatter {
    /// 对应 `DefaultRefMapFormatter(int slotSize, Register fp, int refMapToFPOffset)`。
    pub fn new(slot_size: i32, fp: Register, ref_map_to_fp_offset: i32) -> Self {
        Self {
            slot_size,
            fp,
            ref_map_to_fp_offset,
        }
    }
}

impl RefMapFormatter for DefaultRefMapFormatter {
    fn format_stack_slot(&self, frame_ref_map_index: i32) -> String {
        let ref_map_offset = frame_ref_map_index * self.slot_size;
        let fp_offset = ref_map_offset + self.ref_map_to_fp_offset;
        if fp_offset >= 0 {
            format!("{}+{}", self.fp, fp_offset)
        } else {
            format!("{}{}", self.fp.name, fp_offset)
        }
    }
}

/// 对应 `static class NumberedRefMapFormatter implements RefMapFormatter`。
#[derive(Debug, Clone, Copy, Default)]
pub struct NumberedRefMapFormatter;

impl NumberedRefMapFormatter {
    /// 对应默认构造器。
    pub fn new() -> Self {
        Self
    }

    /// 对应 `formatRegister(int regRefMapIndex)`。
    pub fn format_register(&self, reg_ref_map_index: i32) -> String {
        format!("r{}", reg_ref_map_index)
    }
}

impl RefMapFormatter for NumberedRefMapFormatter {
    fn format_stack_slot(&self, frame_ref_map_index: i32) -> String {
        format!("s{}", frame_ref_map_index)
    }
}

/// 对应 `static StringBuilder append(StringBuilder sb, DebugInfo info, RefMapFormatter formatterArg)`。
pub fn append_debug_info(
    buf: &mut String,
    info: &DebugInfo,
    formatter: Option<&dyn RefMapFormatter>,
) {
    let numbered = NumberedRefMapFormatter::new();
    let formatter: &dyn RefMapFormatter = match formatter {
        Some(f) => f,
        None => &numbered,
    };
    if let Some(ref_map) = info.get_reference_map() {
        // Java `refMap.toString()`：`ReferenceMap` 为抽象类，具体子类覆写 toString；Rust 侧
        // `ReferenceMap: Debug`（无 Display），用 `{:?}` 对齐字符串化（偏离，见模块偏离记录）。
        let _ = write!(buf, "{:?}", ref_map);
    }
    if let Some(callee_save_info) = info.get_callee_save_info() {
        buf.push_str("callee-save-info:");
        buf.push_str(NEW_LINE);
        let map = callee_save_info.slots_to_registers(true);
        for (slot, reg) in &map {
            let _ = write!(
                buf,
                "    {} -> {}{}",
                reg,
                formatter.format_stack_slot(*slot),
                NEW_LINE
            );
        }
    }
    if let Some(frame_pos) = info.frame() {
        append_frame_inner(buf, frame_pos);
    } else {
        append_position(buf, info.get_bytecode_position());
    }
}

/// 对应 `static CallingConvention getCallingConvention(CodeCacheProvider, CallingConvention.Type, ResolvedJavaMethod, ValueKindFactory<?>)`。
pub fn get_calling_convention(
    code_cache: &dyn CodeCacheProvider,
    type_: &dyn CallingConventionType,
    method: &dyn ResolvedJavaMethod,
    value_kind_factory: &dyn ValueKindFactory,
) -> CallingConvention {
    let sig = method.get_signature();
    let ret_type = sig.get_return_type(None);
    let sig_count = sig.get_parameter_count(false);
    let mut owned_arg_types: Vec<Box<dyn JavaType>> = Vec::with_capacity(sig_count as usize + 1);
    let mut arg_types: Vec<&dyn JavaType> = Vec::with_capacity(sig_count as usize + 1);
    if !method.is_static() {
        arg_types.push(method.get_declaring_class());
    }
    for i in 0..sig_count {
        owned_arg_types.push(sig.get_parameter_type(i, None));
    }
    for owned in &owned_arg_types {
        arg_types.push(owned.as_ref());
    }
    let register_config = code_cache.get_register_config();
    register_config.get_calling_convention(
        type_,
        Some(ret_type.as_ref()),
        arg_types,
        value_kind_factory,
    )
}
