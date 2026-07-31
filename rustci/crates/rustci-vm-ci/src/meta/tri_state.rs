// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2012, 2015, Oracle and/or its affiliates. All rights reserved.
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

/// 镜像 `jdk.vm.ci.meta.TriState`：可取 `True`/`False`/`Unknown` 的逻辑值。
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TriState {
    True,
    False,
    Unknown,
}

impl TriState {
    pub fn get(value: bool) -> Self {
        if value {
            TriState::True
        } else {
            TriState::False
        }
    }

    /// 对 `Unknown` 持乐观（已知优先于 Unknown），对已知取悲观（True 优先于 False）。
    pub fn merge(a: TriState, b: TriState) -> TriState {
        if a == TriState::True || b == TriState::True {
            return TriState::True;
        }
        if a == TriState::False || b == TriState::False {
            return TriState::False;
        }
        debug_assert!(a == TriState::Unknown && b == TriState::Unknown);
        TriState::Unknown
    }

    pub fn is_true(self) -> bool {
        self == TriState::True
    }

    pub fn is_false(self) -> bool {
        self == TriState::False
    }

    pub fn is_unknown(self) -> bool {
        self == TriState::Unknown
    }

    pub fn is_known(self) -> bool {
        self != TriState::Unknown
    }

    pub fn to_boolean(self) -> bool {
        if self.is_true() {
            true
        } else if self.is_false() {
            false
        } else {
            panic!("Cannot convert to boolean, TriState is in an unknown state");
        }
    }

    /// 对应 Java `Enum.name()`：返回 Java 枚举常量名（`TRUE`/`FALSE`/`UNKNOWN`）。
    /// Rust 变体名用 PascalCase，Java 用 UPPERCASE；`name()` 对齐 Java 输出。
    pub fn name(self) -> &'static str {
        match self {
            TriState::True => "TRUE",
            TriState::False => "FALSE",
            TriState::Unknown => "UNKNOWN",
        }
    }
}

impl std::fmt::Display for TriState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
