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
        // Mirrors the cursor type hierarchy (EconomicMapWrapCursor IS a
        // MapCursor), but the real remove/setValue semantics of
        // EconomicMapWrap's cursor (which delegate to the underlying
        // iterator.remove / entry.setValue in Java) are deferred to the full
        // EconomicMapImpl port. The stand-in cursor borrows the map
        // immutably (Java's getEntries is non-mutating) and cannot mutate.
        panic!("UnsupportedOperationException: EconomicMapWrapCursor.remove is deferred to the EconomicMapImpl port");
    }
}
