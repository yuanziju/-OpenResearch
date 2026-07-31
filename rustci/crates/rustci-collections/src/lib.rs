/*
 * Copyright (c) 2017, 2026, Oracle and/or its affiliates. All rights reserved.
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
//
// Rust mirror of `org.graalvm.collections` (sdk module). Faithful 1:1 port of the
// interface surface (UnmodifiableEconomicMap / EconomicMap / MapCursor /
// UnmodifiableMapCursor / Equivalence / Pair / EconomicSet / UnmodifiableEconomicSet)
// plus a BTreeMap-based reference implementation (`BTreeEconomicMap` /
// `BTreeEconomicSet`). The full segmented-array `EconomicMapImpl` (941 LoC) is
// deferred to a later task; this crate stands in until then.

pub mod btree_economic_map;
pub mod btree_economic_set;
pub mod economic_map;
pub mod economic_map_wrap;
pub mod economic_set;
pub mod empty;
pub mod equivalence;
pub mod map_cursor;
pub mod pair;

pub use btree_economic_map::{BTreeEconomicMap, BTreeEconomicMapCursor};
pub use btree_economic_set::BTreeEconomicSet;
pub use economic_map::{EconomicMap, UnmodifiableEconomicMap};
pub use economic_map_wrap::EconomicMapWrap;
pub use economic_set::{EconomicSet, UnmodifiableEconomicSet};
pub use empty::{EmptyCursor, EmptyMap, EmptySet};
pub use equivalence::Equivalence;
pub use map_cursor::{MapCursor, UnmodifiableMapCursor};
pub use pair::Pair;

#[cfg(test)]
mod tests;
