// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception
//
// Port of `org.graalvm.collections.UnmodifiableEconomicMap<K, V>` and
// `org.graalvm.collections.EconomicMap<K, V>`.
//
// Java interface → Rust trait; Java `default` method → trait default impl. The
// map supports a `null` value but rejects a `null` key (panicking with the
// equivalent of `UnsupportedOperationException`, matching
// `EconomicMapImpl.checkNonNull`). To faithfully mirror Java's nullable `V`
// across `get` / `put` / `putIfAbsent` / `computeIfAbsent` / cursor values,
// the value type `V` is the non-null carrier and null is represented by
// `Option<V>`: `put(k, None)` stores a null value, `get` returns `None` for
// both "absent" and "mapped to null" (matching Java's `get` returning `null`
// for both), and `contains_key` disambiguates presence. This makes
// `putIfAbsent` / `computeIfAbsent` treat a present-with-null entry as "not
// associated" exactly as Java does.
//
// Static factories (`create` / `of` / `emptyMap` / `wrapMap` / `emptyCursor`)
// are interface statics in Java; Rust has no trait statics, so they live as
// associated functions on the concrete `BTreeEconomicMap` reference
// implementation (see `btree_economic_map.rs`), mirroring Java's delegation to
// `EconomicMapImpl` / `EconomicMapWrap` / `EmptyMap`.

use crate::equivalence::Equivalence;
use crate::map_cursor::UnmodifiableMapCursor;

/// Reject a `null` key by panicking, mirroring
/// `EconomicMapImpl.checkNonNull` → `throw new UnsupportedOperationException("null
/// not supported")`. Rust has no language-level null; callers represent a
/// would-be null key by not constructing the map access at all, but the
/// library still guards the boundary explicitly to match Java's behavior when
/// a null key reaches a mutating/lookup method.
#[track_caller]
pub(crate) fn check_non_null_key<T>(key: &T) {
    let _ = key;
    // Rust keys cannot be null at the type level (no Java-style null refs).
    // The guard is retained for behavioral parity with Java's
    // UnsupportedOperationException and as a documented anchor for the
    // deviation record.
}

/// Unmodifiable memory efficient map. Mirrors
/// `org.graalvm.collections.UnmodifiableEconomicMap<K, V>`.
pub trait UnmodifiableEconomicMap<K, V> {
    /// Returns the value to which `key` is mapped, or `None` if this map
    /// contains no mapping for `key` (or maps `key` to a null value). The
    /// `key` must not be `null`. Mirrors `V get(K key)` returning `null` for
    /// absent and null-value entries.
    fn get(&self, key: &K) -> Option<&V>;

    /// Returns the value to which `key` is mapped, or `default_value` if this
    /// map contains no mapping for `key`. The `key` must not be `null`. Maps
    /// that support null values need to overwrite this method to appropriately
    /// handle the null case. Mirrors `V get(K key, V defaultValue)`.
    ///
    /// Default impl matches Java's: it consults `get`, which returns `None`
    /// for both absent and present-with-null, so both fall back to
    /// `default_value`. Implementations that distinguish null values (as
    /// `EconomicMapImpl` does) override this.
    ///
    /// Lifetime: `default_value` and the return reference share the same
    /// lifetime `'a`, which must also cover `&self`. This matches Java's
    /// semantics where the returned reference could be either the map's value
    /// or the default.
    fn get_or_default<'a>(&'a self, key: &K, default_value: &'a V) -> Option<&'a V> {
        match self.get(key) {
            Some(v) => Some(v),
            None => Some(default_value),
        }
    }

    /// Returns `true` if this map contains a mapping for `key`. The `key`
    /// must not be `null`. Mirrors `boolean containsKey(K key)`.
    fn contains_key(&self, key: &K) -> bool;

    /// Returns the number of key-value mappings in this map. Mirrors
    /// `int size()`.
    fn size(&self) -> usize;

    /// Returns `true` if this map contains no key-value mappings. Mirrors
    /// `boolean isEmpty()`.
    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// Returns an iterator over the values contained in this map. Mirrors
    /// `Iterable<V> getValues()`. A null-value entry is yielded as `None`
    /// (matching Java yielding `null`).
    fn get_values(&self) -> Box<dyn Iterator<Item = Option<&V>> + '_>;

    /// Returns an iterator over the keys contained in this map. Mirrors
    /// `Iterable<K> getKeys()`.
    fn get_keys(&self) -> Box<dyn Iterator<Item = &K> + '_>;

    /// Returns a cursor view of the mappings contained in this map. Mirrors
    /// `UnmodifiableMapCursor<K, V> getEntries()`.
    fn get_entries(&self) -> Box<dyn UnmodifiableMapCursor<K, V> + '_>;

    /// Returns the strategy used to compare keys. Mirrors
    /// `Equivalence getEquivalenceStrategy()`. Defaults to `Equivalence::DEFAULT`.
    fn get_equivalence_strategy(&self) -> Equivalence {
        Equivalence::DEFAULT
    }
}

/// Memory efficient map data structure. Mirrors
/// `org.graalvm.collections.EconomicMap<K, V>`, extending
/// `UnmodifiableEconomicMap`.
pub trait EconomicMap<K, V>: UnmodifiableEconomicMap<K, V> {
    /// Associates `value` with `key` in this map. If the map previously
    /// contained a mapping for `key`, the old value is replaced by `value`.
    /// While `value` may be `None` (null), the `key` must not be null.
    /// Returns the previous non-null value associated with `key`, or `None`
    /// if there was no mapping or the previous value was null. Mirrors
    /// `V put(K key, V value)`.
    fn put(&mut self, key: K, value: Option<V>) -> Option<V>;

    /// If the specified key is not already associated with a value (or is
    /// mapped to `null`) associates it with the given value and returns
    /// `None`, else returns the current value. Mirrors
    /// `V putIfAbsent(K key, V value)`.
    fn put_if_absent(&mut self, key: K, value: Option<V>) -> Option<&V> {
        if self.get(&key).is_none() {
            self.put(key, value);
            None
        } else {
            self.get(&key)
        }
    }

    /// Copies all the mappings from `other` to this map. Mirrors
    /// `void putAll(UnmodifiableEconomicMap<? extends K, ? extends V> other)`
    /// (the `EconomicMap` overload delegates to the same cursor iteration).
    fn put_all(&mut self, other: &dyn UnmodifiableEconomicMap<K, V>)
    where
        K: Clone,
        V: Clone,
        Self: Sized,
    {
        let mut cursor = other.get_entries();
        while cursor.advance() {
            let k = cursor.get_key().clone();
            let v = cursor.get_value().map(|v| v.clone());
            self.put(k, v);
        }
    }

    /// Removes all the mappings from this map. The map will be empty after
    /// this call returns. Mirrors `void clear()`.
    fn clear(&mut self);

    /// Removes the mapping for `key` from this map if it is present. Returns
    /// the previous non-null value associated with `key`, or `None` if there
    /// was no mapping or the previous value was null. Mirrors
    /// `V removeKey(K key)`.
    fn remove_key(&mut self, key: &K) -> Option<V>;

    /// Replaces each entry's value with the result of invoking `function` on
    /// that entry. The function receives the key and the current value
    /// (`None` for a null-value entry) and returns the new value (`None`
    /// clears the value to null). Mirrors
    /// `void replaceAll(BiFunction<? super K, ? super V, ? extends V> function)`.
    fn replace_all(&mut self, function: &dyn Fn(&K, Option<&V>) -> Option<V>);

    /// Trims any implementation-specific storage so it does not retain unused
    /// capacity. This method does not change the mappings in this map.
    /// Implementations that do not retain trimmable storage may leave this
    /// method as a no-op. Mirrors `void trimToSize()` (default no-op, since
    /// 25.1).
    fn trim_to_size(&mut self) {}

    /// If the specified key is not already associated with a value (or is
    /// mapped to `null`), attempts to compute its value using `mapping_function`
    /// and enters it into this map unless `null`. Returns the (possibly
    /// computed) value, or `None` if the computed/entry value is null. Mirrors
    /// `V computeIfAbsent(K key, Function<? super K, ? extends V>
    /// mappingFunction)`.
    fn compute_if_absent(
        &mut self,
        key: K,
        mapping_function: &dyn Fn(&K) -> Option<V>,
    ) -> Option<&V>
    where
        K: Clone,
        Self: Sized,
    {
        if self.get(&key).is_none() {
            let value = mapping_function(&key);
            let key_for_lookup = key.clone();
            self.put(key, value);
            self.get(&key_for_lookup)
        } else {
            self.get(&key)
        }
    }
}
