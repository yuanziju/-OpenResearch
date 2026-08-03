/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::io;

use super::data_source::DataSource;
use super::version_mismatch_exception::VersionMismatchException;
use crate::graphio::graph_protocol::MAGIC_BYTES;

/// A DataSource implementation backed by a byte slice.
pub struct BinarySource {
    data: Vec<u8>,
    pos: usize,
    major_version: u8,
    minor_version: u8,
    mark: u64,
    digest_active: bool,
    digest_data: Vec<u8>,
}

impl BinarySource {
    pub fn new(data: Vec<u8>) -> Self {
        BinarySource {
            data,
            pos: 0,
            major_version: 0,
            minor_version: 0,
            mark: 0,
            digest_active: false,
            digest_data: Vec::new(),
        }
    }

    fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        if self.pos + buf.len() > self.data.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Unexpected end of stream",
            ));
        }
        buf.copy_from_slice(&self.data[self.pos..self.pos + buf.len()]);
        if self.digest_active {
            self.digest_data.extend_from_slice(buf);
        }
        self.pos += buf.len();
        Ok(())
    }

    fn read_fixed<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        let mut buf = [0u8; N];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    pub fn major_version(&self) -> u8 {
        self.major_version
    }

    pub fn minor_version(&self) -> u8 {
        self.minor_version
    }
}

impl DataSource for BinarySource {
    fn read_header(&mut self) -> io::Result<bool> {
        if self.remaining() < 4 {
            return Ok(false);
        }
        let magic = self.read_fixed::<4>()?;
        if magic != *MAGIC_BYTES {
            return Err(
                VersionMismatchException::new("Bad magic bytes in BGV header".to_string()).into(),
            );
        }
        if self.remaining() < 2 {
            return Ok(false);
        }
        self.major_version = self.read_byte()?;
        self.minor_version = self.read_byte()?;
        Ok(true)
    }

    fn read_byte(&mut self) -> io::Result<u8> {
        let buf = self.read_fixed::<1>()?;
        Ok(buf[0])
    }

    fn read_int(&mut self) -> io::Result<i32> {
        let buf = self.read_fixed::<4>()?;
        Ok(i32::from_be_bytes(buf))
    }

    fn read_doubles(&mut self) -> io::Result<Vec<f64>> {
        let len = self.read_int()? as usize;
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(self.read_double()?);
        }
        Ok(result)
    }

    fn read_ints(&mut self) -> io::Result<Vec<i32>> {
        let len = self.read_int()? as usize;
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(self.read_int()?);
        }
        Ok(result)
    }

    fn read_short(&mut self) -> io::Result<u16> {
        let buf = self.read_fixed::<2>()?;
        Ok(u16::from_be_bytes(buf))
    }

    fn read_long(&mut self) -> io::Result<i64> {
        let buf = self.read_fixed::<8>()?;
        Ok(i64::from_be_bytes(buf))
    }

    fn read_float(&mut self) -> io::Result<f32> {
        let buf = self.read_fixed::<4>()?;
        Ok(f32::from_be_bytes(buf))
    }

    fn read_double(&mut self) -> io::Result<f64> {
        let buf = self.read_fixed::<8>()?;
        Ok(f64::from_be_bytes(buf))
    }

    fn read_string(&mut self) -> io::Result<String> {
        let bytes = self.read_bytes()?;
        String::from_utf8(bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    fn read_bytes(&mut self) -> io::Result<Vec<u8>> {
        let len = self.read_short()? as usize;
        let mut buf = vec![0u8; len];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_bytes_len(&mut self, len: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; len];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn get_mark(&self) -> u64 {
        self.mark
    }

    fn get_major_version(&self) -> u8 {
        self.major_version
    }

    fn start_digest(&mut self) {
        self.digest_active = true;
        self.digest_data.clear();
    }

    fn finish_digest(&mut self) -> Vec<u8> {
        self.digest_active = false;
        std::mem::take(&mut self.digest_data)
    }
}
