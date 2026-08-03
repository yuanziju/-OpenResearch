/*
 * Copyright (c) 2011, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 *
 * Subject to the condition set forth below, permission is hereby granted to any
 * person obtaining a copy of this software, associated documentation and/or
 * data (collectively the "Software"), free of charge and under any and all
 * copyright rights in the Software, and any and all patent rights owned or
 * freely licensable by each licensor hereunder covering either (i) the
 * unmodified Software as contributed to or provided by such licensor, or (ii)
 * the Larger Works (as defined below), to deal in both
 *
 * (a) the Software, and
 *
 * (b) any piece of software and/or hardware listed in the lrgrwrks.txt file if
 * one is included with the Software each a "Larger Work" to which the Software
 * is contributed by such licensors),
 *
 * without restriction, including without limitation the rights to copy, create
 * derivative works of, display, perform, and distribute the Software and make,
 * use, sell, offer for sale, import, export, have made, and have sold the
 * Software and the Larger Work(s), and to sublicense the foregoing rights on
 * either these or other terms.
 *
 * This license is subject to the following condition:
 *
 * The above copyright notice and either this complete permission notice or at a
 * minimum a reference to the UPL must be included in all copies or substantial
 * portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Magic bytes identifying a BGV (Binary Graph Viewer) file.
pub const MAGIC_BYTES: &[u8; 4] = b"BIGV";

/// Maximum protocol major version supported.
pub const MAJOR_VERSION: u8 = 8;
/// Maximum protocol minor version supported.
pub const MINOR_VERSION: u8 = 0;

/// Maximum size of the constant pool.
pub const CONSTANT_POOL_MAX_SIZE: usize = 8000;

// --- Stream operation codes ---
pub const BEGIN_GROUP: u8 = 0x00;
pub const BEGIN_GRAPH: u8 = 0x01;
pub const CLOSE_GROUP: u8 = 0x02;
pub const BEGIN_DOCUMENT: u8 = 0x03;

// --- Pool entry types ---
pub const POOL_NEW: u8 = 0x00;
pub const POOL_STRING: u8 = 0x01;
pub const POOL_ENUM: u8 = 0x02;
pub const POOL_CLASS: u8 = 0x03;
pub const POOL_METHOD: u8 = 0x04;
pub const POOL_NULL: u8 = 0x05;
pub const POOL_NODE_CLASS: u8 = 0x06;
pub const POOL_FIELD: u8 = 0x07;
pub const POOL_SIGNATURE: u8 = 0x08;
pub const POOL_NODE_SOURCE_POSITION: u8 = 0x09;
pub const POOL_NODE: u8 = 0x0a;

// --- Property type codes ---
pub const PROPERTY_POOL: u8 = 0x00;
pub const PROPERTY_INT: u8 = 0x01;
pub const PROPERTY_LONG: u8 = 0x02;
pub const PROPERTY_DOUBLE: u8 = 0x03;
pub const PROPERTY_FLOAT: u8 = 0x04;
pub const PROPERTY_TRUE: u8 = 0x05;
pub const PROPERTY_FALSE: u8 = 0x06;
pub const PROPERTY_ARRAY: u8 = 0x07;
pub const PROPERTY_SUBGRAPH: u8 = 0x08;

// --- Class kind codes ---
pub const KLASS: u8 = 0x00;
pub const ENUM_KLASS: u8 = 0x01;

/// Name of the stream attribute that identifies the VM execution.
pub const ATTR_VM_ID: &str = "vm.uuid";

/// The default major protocol version used by the Builder.
pub const DEFAULT_MAJOR_VERSION: i32 = 8;
/// The default minor protocol version used by the Builder.
pub const DEFAULT_MINOR_VERSION: i32 = 0;
