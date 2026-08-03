/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

pub const BEGIN_GROUP: u8 = 0;
pub const BEGIN_GRAPH: u8 = 1;
pub const CLOSE_GROUP: u8 = 2;
pub const STREAM_PROPERTIES: u8 = 3;

pub const POOL_NEW: u8 = 0;
pub const POOL_STRING: u8 = 1;
pub const POOL_ENUM: u8 = 2;
pub const POOL_CLASS: u8 = 3;
pub const POOL_METHOD: u8 = 4;
pub const POOL_NULL: u8 = 5;
pub const POOL_NODE_CLASS: u8 = 6;
pub const POOL_FIELD: u8 = 7;
pub const POOL_SIGNATURE: u8 = 8;
pub const POOL_NODE_SOURCE_POSITION: u8 = 9;
pub const POOL_NODE: u8 = 10;

pub const KLASS: u8 = 0;
pub const ENUM_KLASS: u8 = 1;
pub const PROPERTY_POOL: u8 = 0;
pub const PROPERTY_INT: u8 = 1;
pub const PROPERTY_LONG: u8 = 2;
pub const PROPERTY_DOUBLE: u8 = 3;
pub const PROPERTY_FLOAT: u8 = 4;
pub const PROPERTY_TRUE: u8 = 5;
pub const PROPERTY_FALSE: u8 = 6;
pub const PROPERTY_ARRAY: u8 = 7;
pub const PROPERTY_SUBGRAPH: u8 = 8;
