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
// Reference implementation of `EconomicMap` backed by `std::collections::BTreeMap`.
//
// This stands in for the segmented-array `EconomicMapImpl` (941 LoC, deferred to
// a later task). Behaviorally it mirrors the observable Java `EconomicMap`
// contract: insertion-ordered iteration (BTreeMap yields keys in `Ord` order;
// Java `EconomicMapImpl` yields in insertion order — see deviation record),
// null-value support via `Option<V>` storage, null-key rejection, lazy
// `computeIfAbsent`, and the `EconomicMapImpl.get(key, defaultValue)` override
// that distinguishes a present-with-null entry from an absent one.
//
// The cursor (`BTreeEconomicMapCursor`) is borrowed immutably from the map
// (mirroring Java's non-mutating `getEntries`), so its `MapCursor::remove` /
// `MapCursor::set_value` are deferred to the `EconomicMapImpl` port (which
// mutates the underlying storage in place). Iteration itself
// (`advance` / `get_key` / `get_value`) is fully implemented.

use std::collections::BTreeMap;

use crate::economic_map::{check_non_null_key, EconomicMap, UnmodifiableEconomicMap};
use crate::economic_map_wrap::EconomicMapWrap;
use crate::empty::{EmptyCursor, EmptyMap};
use crate::equivalence::Equivalence;
use crate::map_cursor::{MapCursor, UnmodifiableMapCursor};

/// Reference `EconomicMap` implementation backed by a `BTreeMap`. Stands in
/// for `EconomicMapImpl` until the latter is ported.
pub struct BTreeEconomicMap<K, V>
where
    K: Ord,
{
    inner: BTreeMap<K, Option<V>>,
    strategy: Equivalence,
}

impl<K, V> BTreeEconomicMap<K, V>
where
    K: Ord,
{
    /// Creates a new empty map with the default `Equivalence::DEFAULT`
    /// strategy. Mirrors `EconomicMap.create()`.
    pub fn create() -> Self {
        Self::create_with_strategy(Equivalence::DEFAULT)
    }

    /// Creates a new empty map with the default strategy and the given initial
    /// capacity hint (BTreeMap does not pre-allocate, so the hint is a no-op;
    /// retained for API parity). Mirrors `EconomicMap.create(int)`.
    pub fn create_with_capacity(_initial_capacity: usize) -> Self {
        Self::create()
    }

    /// Creates a new empty map with the given comparison strategy. Mirrors
    /// `EconomicMap.create(Equivalence)`.
    pub fn create_with_strategy(strategy: Equivalence) -> Self {
        BTreeEconomicMap {
            inner: BTreeMap::new(),
            strategy,
        }
    }

    /// Creates a new empty map with the given strategy and capacity hint.
    /// Mirrors `EconomicMap.create(Equivalence, int)`.
    pub fn create_with_strategy_capacity(strategy: Equivalence, _initial_capacity: usize) -> Self {
        Self::create_with_strategy(strategy)
    }

    /// Creates a new map copying all entries from `m`. Mirrors
    /// `EconomicMap.create(UnmodifiableEconomicMap)`.
    pub fn create_from(m: &dyn UnmodifiableEconomicMap<K, V>) -> Self
    where
        K: Clone,
        V: Clone,
    {
        Self::create_with_strategy_from(Equivalence::DEFAULT, m)
    }

    /// Creates a new map with the given strategy copying all entries from `m`.
    /// Mirrors `EconomicMap.create(Equivalence, UnmodifiableEconomicMap)`.
    pub fn create_with_strategy_from(
        strategy: Equivalence,
        m: &dyn UnmodifiableEconomicMap<K, V>,
    ) -> Self
    where
        K: Clone,
        V: Clone,
    {
        let mut map = Self::create_with_strategy(strategy);
        map.put_all(m);
        map
    }

    /// Adopts an existing `BTreeMap<K, Option<V>>` as the backing store.
    /// Useful when migrating from a raw `BTreeMap` to the `EconomicMap` API.
    pub fn from_btree_map(map: BTreeMap<K, Option<V>>) -> Self {
        BTreeEconomicMap {
            inner: map,
            strategy: Equivalence::DEFAULT,
        }
    }

    /// Wraps an existing `BTreeMap` as an `EconomicMap` via `EconomicMapWrap`.
    /// Mirrors `EconomicMap.wrapMap(Map<K, V>)`; the Rust analog of
    /// `java.util.Map<K, V>` for this stand-in layer is
    /// `BTreeMap<K, Option<V>>` (nullable-aware, consistent with the map's
    /// internal storage).
    pub fn wrap_map(map: BTreeMap<K, Option<V>>) -> EconomicMapWrap<K, V>
    where
        K: Ord,
    {
        EconomicMapWrap::new(map)
    }

    /// Returns an empty, unmodifiable `EconomicMap` singleton view. Mirrors
    /// `EconomicMap.emptyMap()`.
    pub fn empty_map() -> EmptyMap<K, V> {
        EmptyMap::default()
    }

    /// Returns an empty `MapCursor`. Mirrors `EconomicMap.emptyCursor()`.
    pub fn empty_cursor() -> EmptyCursor<K, V> {
        EmptyCursor::default()
    }

    /// Creates an `EconomicMap` with one mapping. Mirrors
    /// `EconomicMap.of(K, V)`.
    pub fn of(key1: K, value1: Option<V>) -> Self {
        let mut map = Self::create_with_capacity(1);
        map.put(key1, value1);
        map
    }

    /// Creates an `EconomicMap` with two mappings. Mirrors
    /// `EconomicMap.of(K, V, K, V)`. (Rust has no method overloading, so the
    /// two-arity overload is named `of2`; deviation recorded.)
    pub fn of2(key1: K, value1: Option<V>, key2: K, value2: Option<V>) -> Self {
        let mut map = Self::create_with_capacity(2);
        map.put(key1, value1);
        map.put(key2, value2);
        map
    }
}

impl<K, V> UnmodifiableEconomicMap<K, V> for BTreeEconomicMap<K, V>
where
    K: Ord,
{
    fn get(&self, key: &K) -> Option<&V> {
        check_non_null_key(key);
        self.inner.get(key).and_then(|opt| opt.as_ref())
    }

    fn get_or_default<'a>(&'a self, key: &K, default_value: &'a V) -> Option<&'a V> {
        // Mirrors EconomicMapImpl.get(K, V): a present-with-null entry returns
        // None (Java null), an absent entry returns the default, and a
        // present-with-value entry returns the value. The trait default
        // (which uses get and conflates null with absent) is overridden.
        check_non_null_key(key);
        match self.inner.get(key) {
            Some(Some(v)) => Some(v),
            Some(None) => None,
            None => Some(default_value),
        }
    }

    fn contains_key(&self, key: &K) -> bool {
        check_non_null_key(key);
        self.inner.contains_key(key)
    }

    fn size(&self) -> usize {
        self.inner.len()
    }

    fn get_values(&self) -> Box<dyn Iterator<Item = Option<&V>> + '_> {
        Box::new(self.inner.values().map(|opt| opt.as_ref()))
    }

    fn get_keys(&self) -> Box<dyn Iterator<Item = &K> + '_> {
        Box::new(self.inner.keys())
    }

    fn get_entries(&self) -> Box<dyn UnmodifiableMapCursor<K, V> + '_> {
        Box::new(BTreeEconomicMapCursor {
            iter: self.inner.iter(),
            current: None,
        })
    }

    fn get_equivalence_strategy(&self) -> Equivalence {
        self.strategy
    }
}

impl<K, V> EconomicMap<K, V> for BTreeEconomicMap<K, V>
where
    K: Ord,
{
    fn put(&mut self, key: K, value: Option<V>) -> Option<V> {
        check_non_null_key(&key);
        match self.inner.insert(key, value) {
            Some(Some(v)) => Some(v),
            Some(None) | None => None,
        }
    }

    fn clear(&mut self) {
        self.inner.clear();
    }

    fn remove_key(&mut self, key: &K) -> Option<V> {
        check_non_null_key(key);
        self.inner.remove(key).and_then(|opt| opt)
    }

    fn replace_all(&mut self, function: &dyn Fn(&K, Option<&V>) -> Option<V>) {
        // Uses iter_mut to avoid snapshotting keys (no K: Clone required).
        for (k, slot) in self.inner.iter_mut() {
            let current = slot.as_ref();
            let new_value = function(k, current);
            *slot = new_value;
        }
    }

    // compute_if_absent uses the trait default, which requires K: Clone.
    // An override using the BTreeMap entry API (avoiding K: Clone) was
    // attempted but the borrow checker (E0515) prevents returning a
    // reference obtained from an OccupiedEntry match binding. The full
    // EconomicMapImpl port will not have this limitation.
}

/// Cursor over a `BTreeEconomicMap`, mirroring the cursor returned by
/// `EconomicMapImpl.getEntries()`. Iteration is over the live entries in
/// `Ord` (sorted) order.
pub struct BTreeEconomicMapCursor<'a, K, V>
where
    K: Ord,
{
    iter: std::collections::btree_map::Iter<'a, K, Option<V>>,
    current: Option<(&'a K, &'a Option<V>)>,
}

impl<'a, K, V> UnmodifiableMapCursor<K, V> for BTreeEconomicMapCursor<'a, K, V>
where
    K: Ord,
{
    fn advance(&mut self) -> bool {
        self.current = self.iter.next();
        self.current.is_some()
    }

    fn get_key(&self) -> &K {
        self.current
            .expect("NoSuchElementException: No current entry")
            .0
    }

    fn get_value(&self) -> Option<&V> {
        self.current
            .expect("NoSuchElementException: No current entry")
            .1
            .as_ref()
    }
}

impl<K, V> MapCursor<K, V> for BTreeEconomicMapCursor<'_, K, V>
where
    K: Ord,
{
    fn remove(&mut self) {
        // Mirrors the cursor type hierarchy (BTreeEconomicMapCursor IS a
        // MapCursor), but the real remove/setValue semantics of
        // EconomicMapImpl's cursor (which mutates the underlying storage in
        // place) are deferred to the full EconomicMapImpl port. The stand-in
        // cursor borrows the map immutably (Java's getEntries is non-mutating)
        // and cannot mutate.
        panic!("UnsupportedOperationException: BTreeEconomicMapCursor.remove is deferred to the EconomicMapImpl port");
    }
}
