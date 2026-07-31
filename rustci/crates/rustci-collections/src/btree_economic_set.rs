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
// Reference implementation of `EconomicSet` backed by
// `std::collections::BTreeSet`.
//
// This stands in for the `EconomicMapImpl`-backed `EconomicSet` (deferred to a
// later task). Behaviorally it mirrors the observable Java `EconomicSet`
// contract: insertion-ordered iteration (BTreeSet yields elements in `Ord`
// order; Java `EconomicMapImpl`-backed sets yield in insertion order — see
// deviation record), null-element rejection, and the full
// `UnmodifiableEconomicSet` / `EconomicSet` method surface.
//
// In Java, `EconomicSet` is implemented by `EconomicMapImpl` using a map with a
// sentinel `SET_PRESENT` value. Here a `BTreeSet<E>` is used directly — simpler
// and behaviorally equivalent for the stand-in layer.

use std::collections::BTreeSet;

use crate::economic_set::{check_non_null_element, EconomicSet, UnmodifiableEconomicSet};
use crate::empty::EmptySet;
use crate::equivalence::Equivalence;

/// Reference `EconomicSet` implementation backed by a `BTreeSet`. Stands in
/// for the `EconomicMapImpl`-backed `EconomicSet` until the former is ported.
pub struct BTreeEconomicSet<E>
where
    E: Ord,
{
    inner: BTreeSet<E>,
    // Retained to mirror `EconomicMapImpl`'s internal strategy storage (Java's
    // `EconomicSet.create(Equivalence)` delegates to `EconomicMapImpl.create(strategy)`).
    // Not read back here because `UnmodifiableEconomicSet` does not expose
    // `getEquivalenceStrategy` (only `UnmodifiableEconomicMap` does); the full
    // `EconomicMapImpl` port will surface it again.
    #[allow(dead_code)]
    strategy: Equivalence,
}

impl<E> BTreeEconomicSet<E>
where
    E: Ord,
{
    /// Creates a new empty set with the default `Equivalence::DEFAULT`
    /// strategy. Mirrors `EconomicSet.create()`.
    pub fn create() -> Self {
        Self::create_with_strategy(Equivalence::DEFAULT)
    }

    /// Creates a new empty set with the default strategy and the given initial
    /// capacity hint (BTreeSet does not pre-allocate, so the hint is a no-op;
    /// retained for API parity). Mirrors `EconomicSet.create(int)`.
    pub fn create_with_capacity(_initial_capacity: usize) -> Self {
        Self::create()
    }

    /// Creates a new empty set with the given comparison strategy. Mirrors
    /// `EconomicSet.create(Equivalence)`.
    pub fn create_with_strategy(strategy: Equivalence) -> Self {
        BTreeEconomicSet {
            inner: BTreeSet::new(),
            strategy,
        }
    }

    /// Creates a new empty set with the given strategy and capacity hint.
    /// Mirrors `EconomicSet.create(Equivalence, int)`.
    pub fn create_with_strategy_capacity(strategy: Equivalence, _initial_capacity: usize) -> Self {
        Self::create_with_strategy(strategy)
    }

    /// Creates a new set copying all elements from `c`. Mirrors
    /// `EconomicSet.create(UnmodifiableEconomicSet)`.
    pub fn create_from(c: &dyn UnmodifiableEconomicSet<E>) -> Self
    where
        E: Clone,
    {
        Self::create_with_strategy_from(Equivalence::DEFAULT, c)
    }

    /// Creates a new set with the given strategy copying all elements from `c`.
    /// Mirrors `EconomicSet.create(Equivalence, UnmodifiableEconomicSet)`.
    pub fn create_with_strategy_from(
        strategy: Equivalence,
        c: &dyn UnmodifiableEconomicSet<E>,
    ) -> Self
    where
        E: Clone,
    {
        let mut set = Self::create_with_strategy(strategy);
        // Mirrors Java's addAll(Iterator) → add delegation, since
        // UnmodifiableEconomicSet is not an EconomicSet in the Rust type
        // hierarchy (trait objects are not substitutable).
        for elem in c.iterator() {
            set.add(elem.clone());
        }
        set
    }

    /// Creates a new set with the default `Equivalence::DEFAULT` comparison
    /// strategy and inserts all elements of `values`. Mirrors
    /// `EconomicSet.create(Iterable<E> c)` (added in 25.1). The Java overload
    /// takes `Iterable<E>`; in Rust a slice is the closest ergonomic analog
    /// (deviation recorded, consistent with `add_all_slice`). The internal
    /// delegation `create() + addAll(values)` mirrors Java's
    /// `EconomicSet.create(Iterable)` which calls `set.addAll(c)` →
    /// `addAll(Iterator)` → `add`.
    pub fn create_from_slice(values: &[E]) -> Self
    where
        E: Clone,
    {
        let mut set = Self::create();
        set.add_all_slice(values);
        set
    }

    /// Returns an empty, unmodifiable `EconomicSet` singleton view. Mirrors
    /// `EconomicSet.emptySet()`.
    pub fn empty_set() -> EmptySet<E> {
        EmptySet::default()
    }

    /// Creates an `EconomicSet` with one element. Mirrors `EconomicSet.of(E)`.
    pub fn of(elem: E) -> Self {
        let mut set = Self::create_with_capacity(1);
        set.add(elem);
        set
    }
}

impl<E> UnmodifiableEconomicSet<E> for BTreeEconomicSet<E>
where
    E: Ord,
{
    fn contains(&self, element: &E) -> bool {
        check_non_null_element(element);
        self.inner.contains(element)
    }

    fn size(&self) -> usize {
        self.inner.len()
    }

    fn iterator(&self) -> Box<dyn Iterator<Item = &E> + '_> {
        Box::new(self.inner.iter())
    }
}

impl<E> EconomicSet<E> for BTreeEconomicSet<E>
where
    E: Ord,
{
    fn add(&mut self, element: E) -> bool {
        check_non_null_element(&element);
        self.inner.insert(element)
    }

    fn remove(&mut self, element: &E) {
        check_non_null_element(element);
        self.inner.remove(element);
    }

    fn clear(&mut self) {
        self.inner.clear();
    }
}
