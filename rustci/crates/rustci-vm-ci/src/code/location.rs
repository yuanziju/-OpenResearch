// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2015, 2024, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.code.Location`：值可存储位置（寄存器或栈槽）。

use crate::code::register::Register;

/// 对应 `final class Location`。
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Location {
    /// 对应 `public final Register reg`。
    pub reg: Option<Register>,
    /// 对应 `public final int offset`。
    pub offset: i32,
}

impl Location {
    /// 私有构造器。
    fn new(reg: Option<Register>, offset: i32) -> Self {
        Self { reg, offset }
    }

    /// 对应 `static Location register(Register reg)`。
    pub fn register(reg: Register) -> Self {
        Self::new(Some(reg), 0)
    }

    /// 对应 `static Location subregister(Register reg, int offset)`。
    pub fn subregister(reg: Register, offset: i32) -> Self {
        Self::new(Some(reg), offset)
    }

    /// 对应 `static Location stack(int offset)`。
    pub fn stack(offset: i32) -> Self {
        Self::new(None, offset)
    }

    /// 对应 `isRegister()`。
    pub fn is_register(&self) -> bool {
        self.reg.is_some()
    }

    /// 对应 `isStack()`。
    pub fn is_stack(&self) -> bool {
        self.reg.is_none()
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg_name = match &self.reg {
            Some(r) => format!("{}:", r.name),
            None => "stack:".to_string(),
        };
        write!(f, "{}{}", reg_name, self.offset)
    }
}
