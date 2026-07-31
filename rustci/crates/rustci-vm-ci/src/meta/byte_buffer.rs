// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2024, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `java.nio.ByteBuffer` 在 `SerializableConstant.serialize` 路径上用到的写入子集。
//!
//! 偏离记录：Java 侧 `ByteBuffer.allocate(...)` 默认大端序；`PrimitiveConstant.serialize`
//! 通过 `put`/`putShort`/`putChar`/`putInt`/`putLong`/`putFloat`/`putDouble` 写入。
//! Rust 侧无标准 `ByteBuffer`，本类型以 `Vec<u8>` 承载、大端序写入，语义对齐 Java
//! `ByteBuffer` 在 JVMCI serialize 路径上的行为。

/// 大端序字节缓冲，对应 `java.nio.ByteBuffer` 写入侧。
pub struct ByteBuffer {
    buf: Vec<u8>,
}

impl ByteBuffer {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
        }
    }

    pub fn put(&mut self, b: i8) {
        self.buf.push(b as u8);
    }

    pub fn put_short(&mut self, s: i16) {
        self.buf.extend_from_slice(&s.to_be_bytes());
    }

    pub fn put_char(&mut self, c: u16) {
        self.buf.extend_from_slice(&c.to_be_bytes());
    }

    pub fn put_int(&mut self, i: i32) {
        self.buf.extend_from_slice(&i.to_be_bytes());
    }

    pub fn put_long(&mut self, l: i64) {
        self.buf.extend_from_slice(&l.to_be_bytes());
    }

    pub fn put_float(&mut self, f: f32) {
        self.buf.extend_from_slice(&f.to_bits().to_be_bytes());
    }

    pub fn put_double(&mut self, d: f64) {
        self.buf.extend_from_slice(&d.to_bits().to_be_bytes());
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.buf
    }
}
