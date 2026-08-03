/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// If thrown from ModelBuilder methods, causes the BinaryReader to skip up to
/// the passed `end` position. The next root element (group or graph) is then
/// processed.
pub struct SkipRootException {
    pub start: u64,
    pub end: u64,
    pub constant_pool: Option<super::constant_pool::ConstantPool>,
}

impl SkipRootException {
    pub fn new(
        start: u64,
        end: u64,
        constant_pool: Option<super::constant_pool::ConstantPool>,
    ) -> Self {
        SkipRootException {
            start,
            end,
            constant_pool,
        }
    }

    pub fn constant_pool(&self) -> Option<&super::constant_pool::ConstantPool> {
        self.constant_pool.as_ref()
    }

    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn end(&self) -> u64 {
        self.end
    }
}

impl std::fmt::Display for SkipRootException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Skip[from {} to {}, pool={}]",
            self.start,
            self.end,
            if self.constant_pool.is_some() {
                "present"
            } else {
                "none"
            }
        )
    }
}

impl std::fmt::Debug for SkipRootException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for SkipRootException {}
