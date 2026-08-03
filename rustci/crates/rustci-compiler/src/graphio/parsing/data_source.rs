/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::io;

/// Interface for reading binary graph data from an input source.
pub trait DataSource {
    fn read_header(&mut self) -> io::Result<bool>;

    fn read_byte(&mut self) -> io::Result<u8>;

    fn read_int(&mut self) -> io::Result<i32>;

    fn read_doubles(&mut self) -> io::Result<Vec<f64>>;

    fn read_ints(&mut self) -> io::Result<Vec<i32>>;

    fn read_short(&mut self) -> io::Result<u16>;

    fn read_long(&mut self) -> io::Result<i64>;

    fn read_float(&mut self) -> io::Result<f32>;

    fn read_double(&mut self) -> io::Result<f64>;

    fn read_string(&mut self) -> io::Result<String>;

    fn read_bytes(&mut self) -> io::Result<Vec<u8>>;

    fn read_bytes_len(&mut self, len: usize) -> io::Result<Vec<u8>>;

    fn get_mark(&self) -> u64;

    fn get_major_version(&self) -> u8;

    fn start_digest(&mut self);

    fn finish_digest(&mut self) -> Vec<u8>;
}
