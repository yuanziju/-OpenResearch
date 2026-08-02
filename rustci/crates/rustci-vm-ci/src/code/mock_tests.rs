// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2024, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES IN THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy has been included in the LICENSE file
 * accompanied by this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit oracle.com if you need additional information or have any
 * questions.
 */

//! `code` 模块最小 mock 测试：验证核心类型构造、常量与行为。
//!
//! 覆盖：
//! - `Register` 构造与 `NONE` 常量、`Ord` 按 `number`。
//! - `RegisterCategory` `PartialEq` 按 `name`。
//! - `StackSlot` 静态工厂 `get`、`as_out_arg`/`as_in_arg` 切换 `add_frame_size`。
//! - `Location` 静态工厂 `register`/`subregister`/`stack` 与判别方法。
//! - `MemoryBarriers::barriers_string` 位掩码格式化。
//! - `InfopointReason` `Ord`（按 Java ordinal）。
//! - `DataSectionReference` 哨兵值 `0xDEADDEAD` 与 `set_offset` 单次赋值语义。
//! - `BytecodeFrame` placeholder BCI 常量与 `is_placeholder_bci`/`get_placeholder_bci_name`。

use crate::code::bytecode_frame::{
    get_placeholder_bci_name, is_placeholder_bci, AFTER_BCI, AFTER_EXCEPTION_BCI, BEFORE_BCI,
    INVALID_FRAMESTATE_BCI, UNKNOWN_BCI, UNWIND_BCI,
};
use crate::code::location::Location;
use crate::code::memory_barriers::{
    barriers_string, JMM_POST_VOLATILE_READ, JMM_POST_VOLATILE_WRITE, JMM_PRE_VOLATILE_READ,
    JMM_PRE_VOLATILE_WRITE, LOAD_LOAD, LOAD_STORE, STORE_LOAD, STORE_STORE,
};
use crate::code::register::{Register, RegisterCategory, NONE, SPECIAL};
use crate::code::site::{DataSectionReference, InfopointReason};
use crate::code::stack_slot::StackSlot;
use crate::meta::value_kind::IllegalValueKind;

#[test]
fn register_construction_and_none() {
    // 构造合法寄存器：number/encoding/name/category 各字段保留。
    let cat = RegisterCategory::new("CPU");
    let r = Register::new(3, 7, "rax", cat);
    assert_eq!(r.number, 3);
    assert_eq!(r.encoding(), 7);
    assert_eq!(r.name, "rax");
    assert_eq!(r.get_register_category().name(), "CPU");
    assert!(r.is_valid());
    assert_eq!(r.to_string(), "rax");

    // NONE 常量：number=-1（非法），is_valid() 返回 false。
    assert_eq!(NONE.number, -1);
    assert_eq!(NONE.name, "noreg");
    assert!(!NONE.is_valid());
    assert_eq!(NONE.get_register_category(), SPECIAL);

    // Ord 按 number：NONE(-1) < r(3)。
    assert!(NONE < r);
    assert!(r > NONE);
}

#[test]
fn register_category_equals_by_name() {
    // PartialEq 按 name：同名等价，may_contain_reference 不影响相等。
    let c1 = RegisterCategory::new("CPU");
    let c2 = RegisterCategory::new("CPU");
    let c3 = RegisterCategory::with_reference("CPU", false);
    let c4 = RegisterCategory::new("FPU");
    assert_eq!(c1, c2);
    assert_eq!(c1, c3);
    assert_ne!(c1, c4);
    assert_eq!(c1.to_string(), "CPU");

    // SPECIAL 常量。
    assert_eq!(SPECIAL.name(), "SPECIAL");
}

#[test]
fn stack_slot_get_and_arg_conversion() {
    // get(kind, offset, add_frame_size)：add_frame_size=true 且 offset<0 表示栈帧内槽。
    let kind = Box::new(IllegalValueKind);
    let slot = StackSlot::get(kind, -8, true);
    assert_eq!(slot.get_raw_offset(), -8);
    assert!(slot.get_raw_add_frame_size());

    // getOffset(totalFrameSize)：add_frame_size=true 时 offset + totalFrameSize。
    assert_eq!(slot.get_offset(32), 24);

    // is_in_caller_frame：add_frame_size=true && offset>=0。
    let slot2 = StackSlot::get(Box::new(IllegalValueKind), 4, true);
    assert!(slot2.is_in_caller_frame());

    // as_out_arg：返回 add_frame_size=false 的等价槽。
    let out = slot2.as_out_arg();
    assert!(!out.get_raw_add_frame_size());
    assert_eq!(out.get_raw_offset(), 4);

    // as_in_arg：返回 add_frame_size=true 的等价槽。
    let in_arg = out.as_in_arg();
    assert!(in_arg.get_raw_add_frame_size());
    assert_eq!(in_arg.get_raw_offset(), 4);
}

#[test]
fn location_factory_methods() {
    let reg = Register::new(0, 0, "r0", RegisterCategory::new("CPU"));

    // register(reg)：reg=Some, offset=0。
    let loc_reg = Location::register(reg);
    assert!(loc_reg.is_register());
    assert!(!loc_reg.is_stack());
    assert_eq!(loc_reg.offset, 0);

    // subregister(reg, offset)：reg=Some, offset=指定值。
    let loc_sub = Location::subregister(reg, 4);
    assert!(loc_sub.is_register());
    assert_eq!(loc_sub.offset, 4);

    // stack(offset)：reg=None。
    let loc_stk = Location::stack(16);
    assert!(!loc_stk.is_register());
    assert!(loc_stk.is_stack());
    assert_eq!(loc_stk.offset, 16);

    // Display：寄存器位置 "r0:0"，栈位置 "stack:16"。
    assert_eq!(loc_reg.to_string(), "r0:0");
    assert_eq!(loc_stk.to_string(), "stack:16");
}

#[test]
fn memory_barriers_string() {
    // 单屏障位。
    assert_eq!(barriers_string(LOAD_LOAD), "LOAD_LOAD");
    assert_eq!(barriers_string(LOAD_STORE), "LOAD_STORE");
    assert_eq!(barriers_string(STORE_LOAD), "STORE_LOAD");
    assert_eq!(barriers_string(STORE_STORE), "STORE_STORE");

    // 组合屏障（JMM 常量）。
    assert_eq!(
        barriers_string(JMM_PRE_VOLATILE_WRITE),
        "LOAD_STORE STORE_STORE"
    );
    assert_eq!(
        barriers_string(JMM_POST_VOLATILE_WRITE),
        "STORE_LOAD STORE_STORE"
    );
    assert_eq!(barriers_string(JMM_PRE_VOLATILE_READ), "");

    // JMM_POST_VOLATILE_READ = LOAD_LOAD | LOAD_STORE。
    assert_eq!(
        barriers_string(JMM_POST_VOLATILE_READ),
        "LOAD_LOAD LOAD_STORE"
    );

    // 全屏障。
    assert_eq!(
        barriers_string(LOAD_LOAD | LOAD_STORE | STORE_LOAD | STORE_STORE),
        "LOAD_LOAD LOAD_STORE STORE_LOAD STORE_STORE"
    );
}

#[test]
fn infopoint_reason_ord() {
    // Ord 按 Java ordinal：SAFEPOINT(0) < CALL(1) < IMPLICIT_EXCEPTION(2) < ...
    assert!(InfopointReason::Safepoint < InfopointReason::Call);
    assert!(InfopointReason::Call < InfopointReason::ImplicitException);
    assert!(InfopointReason::ImplicitException < InfopointReason::MethodStart);
    assert!(InfopointReason::MethodStart < InfopointReason::MethodEnd);
    assert!(InfopointReason::MethodEnd < InfopointReason::BytecodePosition);

    // Display 对齐 Java Enum.name。
    assert_eq!(InfopointReason::Safepoint.to_string(), "SAFEPOINT");
    assert_eq!(InfopointReason::Call.to_string(), "CALL");
    assert_eq!(
        InfopointReason::ImplicitException.to_string(),
        "IMPLICIT_EXCEPTION"
    );
}

#[test]
fn data_section_reference_sentinel_and_set_offset() {
    // 新建：initialized=false，Display 显示 "DataSection[?]"（哨兵值 0xDEADDEAD 为内部细节，
    // get_offset 在未初始化时 panic，故仅经 Display 验证未初始化状态）。
    let dsr_uninit = DataSectionReference::new();
    assert_eq!(dsr_uninit.to_string(), "DataSection[?]");

    // Default trait 等价 new。
    let dsr_default = DataSectionReference::default();
    assert_eq!(dsr_default.to_string(), "DataSection[?]");

    // set_offset 单次赋值：首次成功，get_offset 返回赋值值。
    let mut dsr = DataSectionReference::new();
    dsr.set_offset(64);
    assert_eq!(dsr.get_offset(), 64);

    // Display：已初始化时 "DataSection[0x40]"。
    assert_eq!(dsr.to_string(), "DataSection[0x40]");
}

#[test]
#[should_panic(expected = "assertion")]
fn data_section_reference_set_offset_twice_panics() {
    // set_offset 二次赋值触发 assert !initialized panic（对齐 Java assert 语义）。
    let mut dsr = DataSectionReference::new();
    dsr.set_offset(32);
    dsr.set_offset(64);
}

#[test]
fn bytecode_frame_placeholder_bci() {
    // is_placeholder_bci：bci < 0 为 placeholder。
    assert!(is_placeholder_bci(UNKNOWN_BCI));
    assert!(is_placeholder_bci(BEFORE_BCI));
    assert!(is_placeholder_bci(AFTER_BCI));
    assert!(is_placeholder_bci(AFTER_EXCEPTION_BCI));
    assert!(is_placeholder_bci(INVALID_FRAMESTATE_BCI));
    assert!(is_placeholder_bci(UNWIND_BCI));
    assert!(!is_placeholder_bci(0));
    assert!(!is_placeholder_bci(42));

    // get_placeholder_bci_name：各 placeholder BCI 对应名称。
    assert_eq!(get_placeholder_bci_name(UNKNOWN_BCI), "UNKNOWN_BCI");
    assert_eq!(get_placeholder_bci_name(BEFORE_BCI), "BEFORE_BCI");
    assert_eq!(get_placeholder_bci_name(AFTER_BCI), "AFTER_BCI");
    assert_eq!(
        get_placeholder_bci_name(AFTER_EXCEPTION_BCI),
        "AFTER_EXCEPTION_BCI"
    );
    assert_eq!(
        get_placeholder_bci_name(INVALID_FRAMESTATE_BCI),
        "INVALID_FRAMESTATE_BCI"
    );
    assert_eq!(get_placeholder_bci_name(UNWIND_BCI), "UNWIND_BCI");

    // 常量值对齐 Java。
    assert_eq!(UNKNOWN_BCI, -5);
    assert_eq!(UNWIND_BCI, -1);
    assert_eq!(BEFORE_BCI, -2);
    assert_eq!(AFTER_BCI, -3);
    assert_eq!(AFTER_EXCEPTION_BCI, -4);
    assert_eq!(INVALID_FRAMESTATE_BCI, -6);
}
