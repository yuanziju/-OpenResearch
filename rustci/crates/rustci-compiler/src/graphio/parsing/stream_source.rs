/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use super::binary_source::BinarySource;
use super::data_source::DataSource;

/// A DataSource that wraps a `BinarySource` with shared ownership for
/// multi-pass reading.
pub struct StreamSource {
    source: Rc<RefCell<BinarySource>>,
}

impl StreamSource {
    pub fn new(source: BinarySource) -> Self {
        StreamSource {
            source: Rc::new(RefCell::new(source)),
        }
    }

    pub fn from_shared(source: Rc<RefCell<BinarySource>>) -> Self {
        StreamSource { source }
    }

    pub fn inner(&self) -> &Rc<RefCell<BinarySource>> {
        &self.source
    }

    pub fn fork(&self) -> StreamSource {
        StreamSource {
            source: Rc::clone(&self.source),
        }
    }
}

impl DataSource for StreamSource {
    fn read_header(&mut self) -> io::Result<bool> {
        self.source.borrow_mut().read_header()
    }

    fn read_byte(&mut self) -> io::Result<u8> {
        self.source.borrow_mut().read_byte()
    }

    fn read_int(&mut self) -> io::Result<i32> {
        self.source.borrow_mut().read_int()
    }

    fn read_doubles(&mut self) -> io::Result<Vec<f64>> {
        self.source.borrow_mut().read_doubles()
    }

    fn read_ints(&mut self) -> io::Result<Vec<i32>> {
        self.source.borrow_mut().read_ints()
    }

    fn read_short(&mut self) -> io::Result<u16> {
        self.source.borrow_mut().read_short()
    }

    fn read_long(&mut self) -> io::Result<i64> {
        self.source.borrow_mut().read_long()
    }

    fn read_float(&mut self) -> io::Result<f32> {
        self.source.borrow_mut().read_float()
    }

    fn read_double(&mut self) -> io::Result<f64> {
        self.source.borrow_mut().read_double()
    }

    fn read_string(&mut self) -> io::Result<String> {
        self.source.borrow_mut().read_string()
    }

    fn read_bytes(&mut self) -> io::Result<Vec<u8>> {
        self.source.borrow_mut().read_bytes()
    }

    fn read_bytes_len(&mut self, len: usize) -> io::Result<Vec<u8>> {
        self.source.borrow_mut().read_bytes_len(len)
    }

    fn get_mark(&self) -> u64 {
        self.source.borrow().get_mark()
    }

    fn get_major_version(&self) -> u8 {
        self.source.borrow().get_major_version()
    }

    fn start_digest(&mut self) {
        self.source.borrow_mut().start_digest()
    }

    fn finish_digest(&mut self) -> Vec<u8> {
        self.source.borrow_mut().finish_digest()
    }
}
