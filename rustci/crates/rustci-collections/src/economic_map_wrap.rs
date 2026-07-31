/*
 * Copyright (c) 2021, 2026, Oracle and/or its affiliates. All rights reserved.
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
// Port of `org.graalvm.collections.EconomicMapWrap<K, V>`.
//
// Java `EconomicMapWrap` wraps an existing `java.util.Map<K, V>` as an
// `EconomicMap`. The Rust analog wraps a `BTreeMap<K, Option<V>>` — the same
// nullable-aware storage used by `BTreeEconomicMap`'s internals — so that null
// values are representable consistently across the crate. Each method delegates
// to the underlying `BTreeMap`, matching the Java delegation pattern.
//
// The cursor (`EconomicMapWrapCursor`) mirrors Java's anonymous `MapCursor`
// returned by `getEntries()`. As with `BTreeEconomicMapCursor`, the stand-in
// borrows the map immutably (Java's `getEntries` is non-mutating), so
// `MapCursor::remove` / `MapCursor::set_value` are deferred to the
// `EconomicMapImpl` port.

use std::collections::BTreeMap;

use crate::economic_map::{check_non_null_key, EconomicMap, UnmodifiableEconomicMap};
use crate::equivalence::Equivalence;
use crate::map_cursor::{MapCursor, UnmodifiableMapCursor};

/// Wraps an existing `BTreeMap<K, Option<V>>` as an `EconomicMap`. Mirrors
/// `org.graalvm.collections.EconomicMapWrap<K, V>`, which wraps a
/// `java.util.Map<K, V>`.
pub struct EconomicMapWrap<K, V>
where
    K: Ord,
{
    map: BTreeMap<K, Option<V>>,
}

impl<K, V> EconomicMapWrap<K, V>
where
    K: Ord,
{
    /// Wraps `map` as an `EconomicMap`. Mirrors `EconomicMapWrap(Map<K, V>)`.
    pub fn new(map: BTreeMap<K, Option<V>>) -> Self {
        EconomicMapWrap { map }
    }
}

impl<K, V> UnmodifiableEconomicMap<K, V> for EconomicMapWrap<K, V>
where
    K: Ord,
{
    fn get(&self, key: &K) -> Option<&V> {
        check_non_null_key(key);
        self.map.get(key).and_then(|opt| opt.as_ref())
    }

    fn get_or_default<'a>(&'a self, key: &K, default_value: &'a V) -> Option<&'a V> {
        // Mirrors EconomicMapWrap.get(K, V) delegating to
        // Map.getOrDefault: a present-with-null entry returns None (Java null),
        // an absent entry returns the default. The trait default (which uses
        // get and conflates null with absent) is overridden to match Java's
        // Map.getOrDefault semantics exactly.
        check_non_null_key(key);
        match self.map.get(key) {
            Some(Some(v)) => Some(v),
            Some(None) => None,
            None => Some(default_value),
        }
    }

    fn contains_key(&self, key: &K) -> bool {
        check_non_null_key(key);
        self.map.contains_key(key)
    }

    fn size(&self) -> usize {
        self.map.len()
    }

    fn get_values(&self) -> Box<dyn Iterator<Item = Option<&V>> + '_> {
        Box::new(self.map.values().map(|opt| opt.as_ref()))
    }

    fn get_keys(&self) -> Box<dyn Iterator<Item = &K> + '_> {
        Box::new(self.map.keys())
    }

    fn get_entries(&self) -> Box<dyn UnmodifiableMapCursor<K, V> + '_> {
        Box::new(EconomicMapWrapCursor {
            iter: self.map.iter(),
            current: None,
        })
    }

    fn get_equivalence_strategy(&self) -> Equivalence {
        Equivalence::DEFAULT
    }
}

impl<K, V> EconomicMap<K, V> for EconomicMapWrap<K, V>
where
    K: Ord,
{
    fn put(&mut self, key: K, value: Option<V>) -> Option<V> {
        check_non_null_key(&key);
        match self.map.insert(key, value) {
            Some(Some(v)) => Some(v),
            Some(None) | None => None,
        }
    }

    fn clear(&mut self) {
        self.map.clear();
    }

    fn remove_key(&mut self, key: &K) -> Option<V> {
        check_non_null_key(key);
        self.map.remove(key).and_then(|opt| opt)
    }

    fn replace_all(&mut self, function: &dyn Fn(&K, Option<&V>) -> Option<V>) {
        for (k, slot) in self.map.iter_mut() {
            let current = slot.as_ref();
            let new_value = function(k, current);
            *slot = new_value;
        }
    }
}

/// Cursor over an `EconomicMapWrap`, mirroring the anonymous `MapCursor`
/// returned by `EconomicMapWrap.getEntries()`. Iteration is over the live
/// entries in `Ord` (sorted) order.
pub struct EconomicMapWrapCursor<'a, K, V>
where
    K: Ord,
{
    iter: std::collections::btree_map::Iter<'a, K, Option<V>>,
    current: Option<(&'a K, &'a Option<V>)>,
}

impl<'a, K, V> UnmodifiableMapCursor<K, V> for EconomicMapWrapCursor<'a, K, V>
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

impl<K, V> MapCursor<K, V> for EconomicMapWrapCursor<'_, K, V>
where
    K: Ord,
{
    fn remove(&mut self) {
        // GAP (deviation record §2): Java's `EconomicMapWrap.getEntries()`
        // returns a cursor whose `remove()` delegates to the underlying
        // `Iterator.remove()` and `setValue()` to `Map.Entry.setValue()`. The
        // stand-in cursor borrows the map immutably (Java's `getEntries` is
        // non-mutating, and the trait `UnmodifiableEconomicMap::get_entries`
        // is `&self`), so it cannot mutate the backing `BTreeMap`. Mutating
        // cursor support requires either a `&mut self` `get_entries` (breaks
        // the trait mirror) or interior mutability (`RefCell`, deviates from
        // Java's storage model); both are deferred. Unlike
        // `BTreeEconomicMap` (where the real semantics land with the
        // `EconomicMapImpl` port), this is a true current gap, not a planned
        // deferral.
        panic!("UnsupportedOperationException: EconomicMapWrapCursor.remove is not yet implemented (gap: Java EconomicMapWrap cursor supports remove via iterator.remove)");
    }

    fn set_value(&mut self, _new_value: Option<V>) -> Option<V> {
        // GAP (see `remove` above): Java's `EconomicMapWrap` cursor supports
        // `setValue` via `Map.Entry.setValue`. Not yet implemented here for
        // the same borrow-checker reason documented on `remove`.
        panic!("UnsupportedOperationException: EconomicMapWrapCursor.set_value is not yet implemented (gap: Java EconomicMapWrap cursor supports setValue via entry.setValue)");
    }
}
