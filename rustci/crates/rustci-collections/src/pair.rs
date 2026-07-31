/*
 * Copyright (c) 2017, 2020, Oracle and/or its affiliates. All rights reserved.
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
// Port of `org.graalvm.collections.Pair<L, R>`.
//
// Java `Pair` is a `final` value class with two nullable reference fields
// (`left`, `right`) and a private constructor reached only via the static
// factories `empty` / `createLeft` / `createRight` / `create`. Both fields are
// nullable, so they are mirrored as `Option<L>` / `Option<R>`. The factories
// take `Option<L>` / `Option<R>` to faithfully represent Java's nullable `L` /
// `R` parameters. `hashCode`, `equals`, `toString` are ported with the Java
// semantics (`Objects.hashCode(null) == 0`; null prints as `"null"`).

use core::fmt;
use core::hash::{Hash, Hasher};

/// Utility class representing a pair of values. Mirrors
/// `org.graalvm.collections.Pair<L, R>`.
#[derive(Debug)]
pub struct Pair<L, R> {
    left: Option<L>,
    right: Option<R>,
}

impl<L, R> Pair<L, R> {
    /// Returns an empty pair. Mirrors `Pair.empty()`.
    pub fn empty() -> Self {
        Pair {
            left: None,
            right: None,
        }
    }

    /// Constructs a pair with its left value being `left`, or returns an empty
    /// pair if `left` is `None`. Mirrors `Pair.createLeft(L)`.
    pub fn create_left(left: Option<L>) -> Self {
        match left {
            None => Pair::empty(),
            Some(value) => Pair {
                left: Some(value),
                right: None,
            },
        }
    }

    /// Constructs a pair with its right value being `right`, or returns an
    /// empty pair if `right` is `None`. Mirrors `Pair.createRight(R)`.
    pub fn create_right(right: Option<R>) -> Self {
        match right {
            None => Pair::empty(),
            Some(value) => Pair {
                left: None,
                right: Some(value),
            },
        }
    }

    /// Constructs a pair with its left value being `left` and its right value
    /// being `right`, or returns an empty pair if both are `None`. Mirrors
    /// `Pair.create(L, R)`.
    pub fn create(left: Option<L>, right: Option<R>) -> Self {
        if left.is_none() && right.is_none() {
            Pair::empty()
        } else {
            Pair { left, right }
        }
    }

    /// Returns the left value of this pair. Mirrors `Pair.getLeft()`.
    pub fn get_left(&self) -> Option<&L> {
        self.left.as_ref()
    }

    /// Returns the right value of this pair. Mirrors `Pair.getRight()`.
    pub fn get_right(&self) -> Option<&R> {
        self.right.as_ref()
    }
}

impl<L: Hash, R: Hash> Hash for Pair<L, R> {
    /// Mirrors `Pair.hashCode()`: `Objects.hashCode(left) + 31 *
    /// Objects.hashCode(right)`, where `Objects.hashCode(null) == 0`.
    fn hash<H: Hasher>(&self, state: &mut H) {
        let left_hash = self.left.as_ref().map_or(0, |l| {
            let mut h = std::collections::hash_map::DefaultHasher::new();
            l.hash(&mut h);
            h.finish() as i32
        });
        let right_hash = self.right.as_ref().map_or(0, |r| {
            let mut h = std::collections::hash_map::DefaultHasher::new();
            r.hash(&mut h);
            h.finish() as i32
        });
        (left_hash.wrapping_add(31i32.wrapping_mul(right_hash))).hash(state);
    }
}

impl<L: PartialEq, R: PartialEq> PartialEq for Pair<L, R> {
    /// Mirrors `Pair.equals(Object)`: true iff `obj` is a `Pair` with equal
    /// `left` and `right` (Java `Objects.equals`).
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}

impl<L: Eq, R: Eq> Eq for Pair<L, R> {}

impl<L: fmt::Display, R: fmt::Display> fmt::Display for Pair<L, R> {
    /// Mirrors `Pair.toString()`: `"(" + left + ", " + right + ")"`, where a
    /// `null` field renders as `"null"`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("(")?;
        match &self.left {
            Some(l) => l.fmt(f)?,
            None => f.write_str("null")?,
        }
        f.write_str(", ")?;
        match &self.right {
            Some(r) => r.fmt(f)?,
            None => f.write_str("null")?,
        }
        f.write_str(")")
    }
}
