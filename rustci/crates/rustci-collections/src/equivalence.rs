/*
 * Copyright (c) 2017, 2019, Oracle and/or its affiliates. All rights reserved.
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
// Port of `org.graalvm.collections.Equivalence`.
//
// Java `Equivalence` is an abstract class with three static final singleton
// instances (`DEFAULT`, `IDENTITY`, `IDENTITY_WITH_SYSTEM_HASHCODE`) that select
// how keys are compared and hashed. Per the Rust mapping guidance ("枚举式常量 →
// Rust enum 或 const"), it is mirrored here as a `Copy` enum carrying the three
// strategies, with the two abstract methods `equals` / `hashCode` ported as
// generic methods on the enum. Extension via subclassing (Java `protected
// Equivalence()`) is intentionally dropped for the stand-in layer; the full
// `EconomicMapImpl` port is the appropriate place to re-introduce an open
// strategy abstraction if needed.

use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Strategy for comparing two objects. Mirrors the three predefined
/// `Equivalence` constants of `org.graalvm.collections.Equivalence`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Equivalence {
    /// Default equivalence calling `equals` to check equality and `hashCode`
    /// for obtaining hash values. Mirrors `Equivalence.DEFAULT`.
    Default,
    /// Identity equivalence using `==` to check equality and `hashCode` for
    /// obtaining hash values. Mirrors `Equivalence.IDENTITY`.
    Identity,
    /// Identity equivalence using `==` to check equality and
    /// `System.identityHashCode` for obtaining hash values. Mirrors
    /// `Equivalence.IDENTITY_WITH_SYSTEM_HASHCODE`.
    IdentityWithSystemHashCode,
}

impl Equivalence {
    pub const DEFAULT: Equivalence = Equivalence::Default;
    pub const IDENTITY: Equivalence = Equivalence::Identity;
    pub const IDENTITY_WITH_SYSTEM_HASHCODE: Equivalence = Equivalence::IdentityWithSystemHashCode;

    /// Returns `true` if the non-`null` arguments are equal to each other and
    /// `false` otherwise. Mirrors `Equivalence.equals(Object, Object)`.
    ///
    /// * `Default` → value equality via `PartialEq` (Java: `a.equals(b)`).
    /// * `Identity` / `IdentityWithSystemHashCode` → pointer identity via
    ///   `core::ptr::eq` (Java: `a == b`).
    pub fn equals<K: PartialEq + ?Sized>(&self, a: &K, b: &K) -> bool {
        match self {
            Equivalence::Default => a == b,
            Equivalence::Identity | Equivalence::IdentityWithSystemHashCode => core::ptr::eq(a, b),
        }
    }

    /// Returns the hash code of a non-`null` argument. Mirrors
    /// `Equivalence.hashCode(Object)` (Java `int`, 32-bit signed).
    ///
    /// * `Default` / `Identity` → the value's own `Hash` impl, collapsed to a
    ///   32-bit signed `i32` (Java: `o.hashCode()`).
    /// * `IdentityWithSystemHashCode` → the object's address collapsed to a
    ///   32-bit signed `i32` (Java: `System.identityHashCode(o)`).
    pub fn hash_code<K: Hash + ?Sized>(&self, o: &K) -> i32 {
        match self {
            Equivalence::Default | Equivalence::Identity => {
                let mut hasher = DefaultHasher::new();
                o.hash(&mut hasher);
                hasher.finish() as i32
            }
            Equivalence::IdentityWithSystemHashCode => {
                core::ptr::addr_of!(*o) as *const () as usize as i32
            }
        }
    }
}
