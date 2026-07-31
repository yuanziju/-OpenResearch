// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception
//
// Unit tests for the `rustci-collections` crate. Each test group is annotated
// with the Java semantics it validates against
// (`org.graalvm.collections`).

use crate::btree_economic_map::BTreeEconomicMap;
use crate::btree_economic_set::BTreeEconomicSet;
use crate::economic_map::{EconomicMap, UnmodifiableEconomicMap};
use crate::economic_map_wrap::EconomicMapWrap;
use crate::economic_set::{EconomicSet, UnmodifiableEconomicSet};
use crate::empty::{EmptyCursor, EmptyMap, EmptySet};
use crate::equivalence::Equivalence;
use crate::map_cursor::UnmodifiableMapCursor;
use crate::pair::Pair;
use std::cell::Cell;

// ---------------------------------------------------------------------------
// BTreeEconomicMap: null value support
// ---------------------------------------------------------------------------

/// Java: `map.put(k, null)` stores a null value; `get(k)` returns null for
/// both absent and null-value entries (indistinguishable via `get`).
/// `containsKey(k)` disambiguates: true for a present-with-null entry.
#[test]
fn null_value_get_returns_none() {
    let mut map = BTreeEconomicMap::<i32, String>::create();
    map.put(1, None);
    assert_eq!(
        map.get(&1),
        None,
        "get should return None for null-value entry"
    );
}

#[test]
fn null_value_contains_key_true() {
    let mut map = BTreeEconomicMap::<i32, String>::create();
    map.put(1, None);
    assert!(
        map.contains_key(&1),
        "contains_key should be true for null-value entry"
    );
}

#[test]
fn absent_key_contains_key_false() {
    let map = BTreeEconomicMap::<i32, String>::create();
    assert!(
        !map.contains_key(&1),
        "contains_key should be false for absent key"
    );
}

#[test]
fn null_value_get_or_default_returns_none() {
    // Java: EconomicMapImpl.get(key, default) returns null for a
    // present-with-null entry (not the default).
    let mut map = BTreeEconomicMap::<i32, String>::create();
    map.put(1, None);
    let default = "default".to_string();
    assert_eq!(
        map.get_or_default(&1, &default),
        None,
        "get_or_default should return None for present-with-null entry"
    );
}

#[test]
fn absent_key_get_or_default_returns_default() {
    let map = BTreeEconomicMap::<i32, String>::create();
    let default = "default".to_string();
    assert_eq!(
        map.get_or_default(&1, &default),
        Some(&default),
        "get_or_default should return default for absent key"
    );
}

#[test]
fn null_value_remove_key_returns_none() {
    let mut map = BTreeEconomicMap::<i32, String>::create();
    map.put(1, None);
    assert_eq!(
        map.remove_key(&1),
        None,
        "remove_key returns None for null-value entry"
    );
    assert!(!map.contains_key(&1), "entry should be removed");
}

#[test]
fn put_replaces_value() {
    let mut map = BTreeEconomicMap::<i32, String>::create();
    map.put(1, Some("a".to_string()));
    assert_eq!(map.get(&1), Some(&"a".to_string()));
    let prev = map.put(1, Some("b".to_string()));
    assert_eq!(prev, Some("a".to_string()));
    assert_eq!(map.get(&1), Some(&"b".to_string()));
}

#[test]
fn put_replaces_value_with_null() {
    let mut map = BTreeEconomicMap::<i32, String>::create();
    map.put(1, Some("a".to_string()));
    let prev = map.put(1, None);
    assert_eq!(prev, Some("a".to_string()));
    assert_eq!(map.get(&1), None);
    assert!(map.contains_key(&1));
}

// ---------------------------------------------------------------------------
// Null key rejection (Rust type-system anchor)
// ---------------------------------------------------------------------------

/// Rust has no language-level null references, so a "null key" cannot be
/// constructed at the type level. The `check_non_null_key` guard is retained
/// as a behavioral anchor for Java's `UnsupportedOperationException` parity
/// but is a no-op in Rust. This test documents that invariant: the guard
/// does not interfere with normal operation.
#[test]
fn null_key_rejection_is_type_level() {
    let mut map = BTreeEconomicMap::<i32, String>::create();
    map.put(42, Some("answer".to_string()));
    assert_eq!(map.get(&42), Some(&"answer".to_string()));
}

/// EmptyMap.put panics with IllegalArgumentException, mirroring Java's
/// EmptyMap.EMPTY_MAP which throws on any mutation (after the null-key check).
#[test]
#[should_panic(expected = "IllegalArgumentException: Cannot modify the always-empty map")]
fn empty_map_put_panics() {
    let mut map = EmptyMap::<i32, String>::default();
    map.put(1, Some("x".to_string()));
}

#[test]
#[should_panic(expected = "IllegalArgumentException: Cannot modify the always-empty map")]
fn empty_map_clear_panics() {
    let mut map = EmptyMap::<i32, String>::default();
    map.clear();
}

#[test]
#[should_panic(expected = "IllegalArgumentException: Cannot modify the always-empty map")]
fn empty_map_remove_key_panics() {
    let mut map = EmptyMap::<i32, String>::default();
    map.remove_key(&1);
}

// ---------------------------------------------------------------------------
// computeIfAbsent lazy evaluation
// ---------------------------------------------------------------------------

/// Java: for an absent key, the mapping function is invoked and the result
/// is stored.
#[test]
fn compute_if_absent_invokes_for_absent_key() {
    let mut map = BTreeEconomicMap::<i32, String>::create();
    let result = map.compute_if_absent(1, &|k| Some(format!("val-{}", k)));
    assert_eq!(result, Some(&"val-1".to_string()));
    assert_eq!(map.get(&1), Some(&"val-1".to_string()));
}

/// Java: for a present key with a non-null value, the mapping function is NOT
/// invoked.
#[test]
fn compute_if_absent_skips_for_present_key() {
    let mut map = BTreeEconomicMap::<i32, String>::create();
    map.put(1, Some("existing".to_string()));
    let called = Cell::new(false);
    let result = map.compute_if_absent(1, &|_k| {
        called.set(true);
        Some("computed".to_string())
    });
    assert!(
        !called.get(),
        "mapping function should not be called for present key"
    );
    assert_eq!(result, Some(&"existing".to_string()));
}

/// Java: for a present key mapped to null, the mapping function IS invoked
/// (matching Java's `get` returning null for null-value entries).
#[test]
fn compute_if_absent_invokes_for_null_value_entry() {
    let mut map = BTreeEconomicMap::<i32, String>::create();
    map.put(1, None);
    let called = Cell::new(false);
    let result = map.compute_if_absent(1, &|_k| {
        called.set(true);
        Some("computed".to_string())
    });
    assert!(
        called.get(),
        "mapping function should be called for null-value entry"
    );
    assert_eq!(result, Some(&"computed".to_string()));
    assert_eq!(map.get(&1), Some(&"computed".to_string()));
}

/// Java: if the mapping function returns null, the null is stored.
#[test]
fn compute_if_absent_stores_null() {
    let mut map = BTreeEconomicMap::<i32, String>::create();
    let result = map.compute_if_absent(1, &|_| None);
    assert_eq!(result, None);
    assert!(map.contains_key(&1), "null result should still be stored");
}

// ---------------------------------------------------------------------------
// Cursor iteration order
// ---------------------------------------------------------------------------

/// BTreeMap yields keys in `Ord` (sorted) order. Java `EconomicMapImpl`
/// yields in insertion order — this is a documented deviation. The test
/// verifies sorted-order iteration is correct and stable.
#[test]
fn cursor_iterates_in_sorted_order() {
    let mut map = BTreeEconomicMap::<i32, String>::create();
    // Insert in non-sorted order.
    map.put(3, Some("c".to_string()));
    map.put(1, Some("a".to_string()));
    map.put(2, Some("b".to_string()));

    let mut cursor = map.get_entries();
    let mut keys = Vec::new();
    let mut values = Vec::new();
    while cursor.advance() {
        keys.push(*cursor.get_key());
        if let Some(v) = cursor.get_value() {
            values.push(v.clone());
        }
    }
    assert_eq!(
        keys,
        vec![1, 2, 3],
        "BTreeMap deviation: sorted order, not insertion order"
    );
    assert_eq!(
        values,
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
}

#[test]
fn cursor_get_key_value_after_advance() {
    let mut map = BTreeEconomicMap::<i32, String>::create();
    map.put(10, Some("ten".to_string()));
    let mut cursor = map.get_entries();
    assert!(
        cursor.advance(),
        "advance should return true when entry exists"
    );
    assert_eq!(*cursor.get_key(), 10);
    assert_eq!(cursor.get_value(), Some(&"ten".to_string()));
    assert!(
        !cursor.advance(),
        "advance should return false when no more entries"
    );
}

/// Java: cursor.getKey/getValue on an exhausted cursor throws
/// NoSuchElementException.
#[test]
#[should_panic(expected = "NoSuchElementException")]
fn cursor_get_key_without_advance_panics() {
    let map = BTreeEconomicMap::<i32, String>::of(1, Some("a".to_string()));
    let cursor = map.get_entries();
    let _ = cursor.get_key();
}

/// Cursor over a null-value entry yields None for getValue (Java yields
/// null).
#[test]
fn cursor_null_value_entry() {
    let mut map = BTreeEconomicMap::<i32, String>::create();
    map.put(1, None);
    let mut cursor = map.get_entries();
    assert!(cursor.advance());
    assert_eq!(*cursor.get_key(), 1);
    assert_eq!(cursor.get_value(), None);
}

// ---------------------------------------------------------------------------
// putAll merging
// ---------------------------------------------------------------------------

/// Java: putAll copies all entries from other, overwriting existing keys.
#[test]
fn put_all_merges_entries() {
    let mut src = BTreeEconomicMap::<i32, String>::create();
    src.put(1, Some("a".to_string()));
    src.put(2, Some("b".to_string()));

    let mut dst = BTreeEconomicMap::<i32, String>::create();
    dst.put(2, Some("old".to_string()));
    dst.put(3, Some("c".to_string()));
    dst.put_all(&src);

    assert_eq!(dst.size(), 3);
    assert_eq!(dst.get(&1), Some(&"a".to_string()));
    assert_eq!(
        dst.get(&2),
        Some(&"b".to_string()),
        "existing key should be overwritten"
    );
    assert_eq!(dst.get(&3), Some(&"c".to_string()));
}

#[test]
fn put_all_includes_null_values() {
    let mut src = BTreeEconomicMap::<i32, String>::create();
    src.put(1, None);
    src.put(2, Some("b".to_string()));

    let mut dst = BTreeEconomicMap::<i32, String>::create();
    dst.put_all(&src);

    assert_eq!(dst.get(&1), None);
    assert!(dst.contains_key(&1), "null-value entry should be copied");
    assert_eq!(dst.get(&2), Some(&"b".to_string()));
}

// ---------------------------------------------------------------------------
// of(...) factories
// ---------------------------------------------------------------------------

#[test]
fn of_single_entry() {
    let map = BTreeEconomicMap::<i32, String>::of(1, Some("a".to_string()));
    assert_eq!(map.size(), 1);
    assert_eq!(map.get(&1), Some(&"a".to_string()));
}

#[test]
fn of_two_entries() {
    let map =
        BTreeEconomicMap::<i32, String>::of2(1, Some("a".to_string()), 2, Some("b".to_string()));
    assert_eq!(map.size(), 2);
    assert_eq!(map.get(&1), Some(&"a".to_string()));
    assert_eq!(map.get(&2), Some(&"b".to_string()));
}

#[test]
fn of_with_null_value() {
    let map = BTreeEconomicMap::<i32, String>::of(1, None);
    assert_eq!(map.size(), 1);
    assert_eq!(map.get(&1), None);
    assert!(map.contains_key(&1));
}

// ---------------------------------------------------------------------------
// Other EconomicMap methods
// ---------------------------------------------------------------------------

#[test]
fn put_if_absent_new_key() {
    let mut map = BTreeEconomicMap::<i32, String>::create();
    let result = map.put_if_absent(1, Some("a".to_string()));
    assert_eq!(result, None, "putIfAbsent returns None for new key");
    assert_eq!(map.get(&1), Some(&"a".to_string()));
}

#[test]
fn put_if_absent_existing_key() {
    let mut map = BTreeEconomicMap::<i32, String>::create();
    map.put(1, Some("existing".to_string()));
    let result = map.put_if_absent(1, Some("new".to_string()));
    assert_eq!(
        result,
        Some(&"existing".to_string()),
        "putIfAbsent returns current value for existing key"
    );
    assert_eq!(
        map.get(&1),
        Some(&"existing".to_string()),
        "value should not change"
    );
}

#[test]
fn put_if_absent_null_value_entry() {
    // Java: putIfAbsent treats a null-value entry as "absent" and sets the
    // new value.
    let mut map = BTreeEconomicMap::<i32, String>::create();
    map.put(1, None);
    let result = map.put_if_absent(1, Some("new".to_string()));
    assert_eq!(
        result, None,
        "putIfAbsent returns None for null-value entry"
    );
    assert_eq!(map.get(&1), Some(&"new".to_string()));
}

#[test]
fn replace_all_updates_values() {
    let mut map = BTreeEconomicMap::<i32, i32>::create();
    map.put(1, Some(10));
    map.put(2, Some(20));
    map.put(3, None);
    map.replace_all(&|k, v| Some(*k + v.copied().unwrap_or(0)));
    assert_eq!(map.get(&1), Some(&11));
    assert_eq!(map.get(&2), Some(&22));
    assert_eq!(map.get(&3), Some(&3), "null value treated as 0");
}

#[test]
fn replace_all_can_set_null() {
    let mut map = BTreeEconomicMap::<i32, i32>::create();
    map.put(1, Some(10));
    map.replace_all(&|_, _| None);
    assert_eq!(map.get(&1), None);
    assert!(map.contains_key(&1));
}

#[test]
fn clear_empties_map() {
    let mut map = BTreeEconomicMap::<i32, String>::of(1, Some("a".to_string()));
    map.clear();
    assert_eq!(map.size(), 0);
    assert!(map.is_empty());
}

#[test]
fn get_keys_and_values() {
    let mut map = BTreeEconomicMap::<i32, String>::create();
    map.put(1, Some("a".to_string()));
    map.put(2, None);
    map.put(3, Some("c".to_string()));

    let keys: Vec<&i32> = map.get_keys().collect();
    assert_eq!(keys, vec![&1, &2, &3]);

    let values: Vec<Option<&String>> = map.get_values().collect();
    assert_eq!(
        values,
        vec![Some(&"a".to_string()), None, Some(&"c".to_string())]
    );
}

#[test]
fn trim_to_size_is_noop() {
    let mut map = BTreeEconomicMap::<i32, String>::of(1, Some("a".to_string()));
    map.trim_to_size();
    assert_eq!(map.size(), 1);
}

#[test]
fn get_equivalence_strategy_default() {
    let map = BTreeEconomicMap::<i32, String>::create();
    assert_eq!(map.get_equivalence_strategy(), Equivalence::DEFAULT);
}

#[test]
fn get_equivalence_strategy_custom() {
    let map = BTreeEconomicMap::<i32, String>::create_with_strategy(Equivalence::IDENTITY);
    assert_eq!(map.get_equivalence_strategy(), Equivalence::IDENTITY);
}

// ---------------------------------------------------------------------------
// create_from / wrap_map / empty_map factories
// ---------------------------------------------------------------------------

#[test]
fn create_from_copies_entries() {
    let mut src = BTreeEconomicMap::<i32, String>::create();
    src.put(1, Some("a".to_string()));
    src.put(2, None);
    let dst = BTreeEconomicMap::<i32, String>::create_from(&src);
    assert_eq!(dst.size(), 2);
    assert_eq!(dst.get(&1), Some(&"a".to_string()));
    assert!(dst.contains_key(&2));
}

#[test]
fn empty_map_is_unmodifiable() {
    let map: EmptyMap<i32, String> = BTreeEconomicMap::<i32, String>::empty_map();
    assert_eq!(map.size(), 0);
    assert!(map.is_empty());
    assert!(!map.contains_key(&1));
    assert_eq!(map.get(&1), None);
}

#[test]
fn empty_cursor_advances_false() {
    let cursor: EmptyCursor<i32, String> = BTreeEconomicMap::<i32, String>::empty_cursor();
    let mut c = cursor;
    assert!(!c.advance());
}

#[test]
#[should_panic(expected = "NoSuchElementException")]
fn empty_cursor_get_key_panics() {
    let cursor: EmptyCursor<i32, String> = BTreeEconomicMap::<i32, String>::empty_cursor();
    let _ = cursor.get_key();
}

// ---------------------------------------------------------------------------
// EconomicMapWrap
// ---------------------------------------------------------------------------

#[test]
fn wrap_map_delegates() {
    let mut inner = std::collections::BTreeMap::new();
    inner.insert(1, Some("a".to_string()));
    inner.insert(2, None);
    let wrap = EconomicMapWrap::<i32, String>::new(inner);

    assert_eq!(wrap.size(), 2);
    assert_eq!(wrap.get(&1), Some(&"a".to_string()));
    assert_eq!(wrap.get(&2), None);
    assert!(wrap.contains_key(&2));
}

#[test]
fn wrap_map_get_or_default() {
    let mut inner = std::collections::BTreeMap::new();
    inner.insert(1, Some("a".to_string()));
    inner.insert(2, None);
    let wrap = EconomicMapWrap::<i32, String>::new(inner);
    let default = "d".to_string();
    assert_eq!(wrap.get_or_default(&1, &default), Some(&"a".to_string()));
    assert_eq!(
        wrap.get_or_default(&2, &default),
        None,
        "present-with-null returns None"
    );
    assert_eq!(
        wrap.get_or_default(&3, &default),
        Some(&"d".to_string()),
        "absent returns default"
    );
}

#[test]
fn wrap_map_put_and_remove() {
    let inner = std::collections::BTreeMap::new();
    let mut wrap = EconomicMapWrap::<i32, String>::new(inner);
    wrap.put(1, Some("a".to_string()));
    assert_eq!(wrap.get(&1), Some(&"a".to_string()));
    let prev = wrap.put(1, Some("b".to_string()));
    assert_eq!(prev, Some("a".to_string()));
    let removed = wrap.remove_key(&1);
    assert_eq!(removed, Some("b".to_string()));
    assert!(!wrap.contains_key(&1));
}

#[test]
fn wrap_map_cursor() {
    let mut inner = std::collections::BTreeMap::new();
    inner.insert(1, Some("a".to_string()));
    inner.insert(2, None);
    let wrap = EconomicMapWrap::<i32, String>::new(inner);
    let mut cursor = wrap.get_entries();
    assert!(cursor.advance());
    assert_eq!(*cursor.get_key(), 1);
    assert_eq!(cursor.get_value(), Some(&"a".to_string()));
    assert!(cursor.advance());
    assert_eq!(*cursor.get_key(), 2);
    assert_eq!(cursor.get_value(), None);
    assert!(!cursor.advance());
}

// ---------------------------------------------------------------------------
// Equivalence
// ---------------------------------------------------------------------------

#[test]
fn equivalence_default_equals() {
    let eq = Equivalence::DEFAULT;
    assert!(eq.equals(&1, &1));
    assert!(eq.equals(&"abc", &"abc"));
    assert!(!eq.equals(&1, &2));
    assert!(!eq.equals(&"abc", &"abd"));
}

#[test]
fn equivalence_identity_equals() {
    let eq = Equivalence::IDENTITY;
    let s1 = "hello".to_string();
    let s2 = "hello".to_string();
    // s1 == s2 by value but not by identity (different allocations).
    assert!(eq.equals(&s1, &s1), "identity equals: same reference");
    assert!(
        !eq.equals(&s1, &s2),
        "identity equals: different references"
    );
}

#[test]
fn equivalence_identity_with_system_hashcode_equals() {
    let eq = Equivalence::IDENTITY_WITH_SYSTEM_HASHCODE;
    let s1 = "hello".to_string();
    let s2 = "hello".to_string();
    assert!(eq.equals(&s1, &s1));
    assert!(!eq.equals(&s1, &s2));
}

#[test]
fn equivalence_hash_code_default() {
    let eq = Equivalence::DEFAULT;
    let h1 = eq.hash_code(&1);
    let h2 = eq.hash_code(&1);
    assert_eq!(h1, h2, "same value → same hash");
    let h3 = eq.hash_code(&2);
    // Different values may collide, but typically don't.
    assert_ne!(h1, h3, "different values → different hash (typical case)");
}

#[test]
fn equivalence_constants() {
    assert_eq!(Equivalence::DEFAULT, Equivalence::Default);
    assert_eq!(Equivalence::IDENTITY, Equivalence::Identity);
    assert_eq!(
        Equivalence::IDENTITY_WITH_SYSTEM_HASHCODE,
        Equivalence::IdentityWithSystemHashCode
    );
}

// ---------------------------------------------------------------------------
// Pair
// ---------------------------------------------------------------------------

#[test]
fn pair_empty() {
    let p: Pair<i32, String> = Pair::empty();
    assert_eq!(p.get_left(), None);
    assert_eq!(p.get_right(), None);
}

#[test]
fn pair_create_left_some() {
    let p: Pair<i32, String> = Pair::create_left(Some(42));
    assert_eq!(p.get_left(), Some(&42));
    assert_eq!(p.get_right(), None);
}

#[test]
fn pair_create_left_none() {
    let p: Pair<i32, String> = Pair::create_left(None);
    assert_eq!(p.get_left(), None);
    assert_eq!(p.get_right(), None);
}

#[test]
fn pair_create_right_some() {
    let p: Pair<i32, String> = Pair::create_right(Some("hello".to_string()));
    assert_eq!(p.get_left(), None);
    assert_eq!(p.get_right(), Some(&"hello".to_string()));
}

#[test]
fn pair_create_right_none() {
    let p: Pair<i32, String> = Pair::create_right(None);
    assert_eq!(p.get_left(), None);
    assert_eq!(p.get_right(), None);
}

#[test]
fn pair_create_both() {
    let p = Pair::create(Some(1), Some("a".to_string()));
    assert_eq!(p.get_left(), Some(&1));
    assert_eq!(p.get_right(), Some(&"a".to_string()));
}

#[test]
fn pair_create_both_none() {
    let p: Pair<i32, String> = Pair::create(None, None);
    assert_eq!(p.get_left(), None);
    assert_eq!(p.get_right(), None);
}

#[test]
fn pair_equality() {
    let p1 = Pair::create(Some(1), Some("a".to_string()));
    let p2 = Pair::create(Some(1), Some("a".to_string()));
    let p3 = Pair::create(Some(2), Some("a".to_string()));
    assert_eq!(p1, p2);
    assert_ne!(p1, p3);
}

#[test]
fn pair_display() {
    let p = Pair::create(Some(1), Some("a".to_string()));
    assert_eq!(format!("{}", p), "(1, a)");
}

#[test]
fn pair_display_null() {
    let p: Pair<i32, String> = Pair::create_left(Some(1));
    assert_eq!(format!("{}", p), "(1, null)");
}

// ---------------------------------------------------------------------------
// BTreeEconomicSet
// ---------------------------------------------------------------------------

#[test]
fn set_add_new_returns_true() {
    let mut set = BTreeEconomicSet::<i32>::create();
    assert!(set.add(1), "add returns true for new element");
    assert!(!set.add(1), "add returns false for existing element");
}

#[test]
fn set_contains() {
    let mut set = BTreeEconomicSet::<i32>::create();
    set.add(1);
    assert!(set.contains(&1));
    assert!(!set.contains(&2));
}

#[test]
fn set_remove() {
    let mut set = BTreeEconomicSet::<i32>::create();
    set.add(1);
    set.add(2);
    set.remove(&1);
    assert!(!set.contains(&1));
    assert!(set.contains(&2));
    assert_eq!(set.size(), 1);
}

#[test]
fn set_clear() {
    let mut set = BTreeEconomicSet::<i32>::of(1);
    set.add(2);
    set.clear();
    assert_eq!(set.size(), 0);
    assert!(set.is_empty());
}

#[test]
fn set_size_and_is_empty() {
    let set = BTreeEconomicSet::<i32>::create();
    assert_eq!(set.size(), 0);
    assert!(set.is_empty());
}

#[test]
fn set_iterator_sorted_order() {
    let mut set = BTreeEconomicSet::<i32>::create();
    set.add(3);
    set.add(1);
    set.add(2);
    let elems: Vec<&i32> = set.iterator().collect();
    assert_eq!(elems, vec![&1, &2, &3], "BTreeSet deviation: sorted order");
}

#[test]
fn set_of_single() {
    let set = BTreeEconomicSet::<i32>::of(42);
    assert_eq!(set.size(), 1);
    assert!(set.contains(&42));
}

#[test]
fn set_empty_set() {
    let set: EmptySet<i32> = BTreeEconomicSet::<i32>::empty_set();
    assert_eq!(set.size(), 0);
    assert!(set.is_empty());
    assert!(!set.contains(&1));
}

#[test]
#[should_panic(expected = "IllegalArgumentException: Cannot modify the always-empty set")]
fn empty_set_add_panics() {
    let mut set: EmptySet<i32> = BTreeEconomicSet::<i32>::empty_set();
    set.add(1);
}

#[test]
fn set_add_all_set() {
    let mut a = BTreeEconomicSet::<i32>::create();
    a.add(1);
    a.add(2);
    let mut b = BTreeEconomicSet::<i32>::create();
    b.add(2);
    b.add(3);
    a.add_all_set(&b);
    assert_eq!(a.size(), 3);
    assert!(a.contains(&3));
}

#[test]
fn set_add_all_slice() {
    let mut set = BTreeEconomicSet::<i32>::create();
    set.add_all_slice(&[1, 2, 3]);
    assert_eq!(set.size(), 3);
}

#[test]
fn set_remove_all_set() {
    let mut a = BTreeEconomicSet::<i32>::create();
    a.add(1);
    a.add(2);
    a.add(3);
    let mut b = BTreeEconomicSet::<i32>::create();
    b.add(2);
    b.add(3);
    a.remove_all_set(&b);
    assert_eq!(a.size(), 1);
    assert!(a.contains(&1));
    assert!(!a.contains(&2));
    assert!(!a.contains(&3));
}

#[test]
fn set_retain_all() {
    let mut a = BTreeEconomicSet::<i32>::create();
    a.add(1);
    a.add(2);
    a.add(3);
    let mut b = BTreeEconomicSet::<i32>::create();
    b.add(2);
    a.retain_all(&b);
    assert_eq!(a.size(), 1);
    assert!(a.contains(&2));
}

#[test]
fn set_contains_all() {
    let mut set = BTreeEconomicSet::<i32>::create();
    set.add(1);
    set.add(2);
    set.add(3);
    assert!(set.contains_all(&[1, 2]));
    assert!(!set.contains_all(&[1, 4]));
}

#[test]
fn set_to_list() {
    let mut set = BTreeEconomicSet::<i32>::create();
    set.add(3);
    set.add(1);
    set.add(2);
    let list = set.to_list();
    assert_eq!(list, vec![1, 2, 3]);
}

#[test]
fn set_to_hash_set() {
    let mut set = BTreeEconomicSet::<i32>::create();
    set.add(1);
    set.add(2);
    let hs = set.to_hash_set();
    assert_eq!(hs.len(), 2);
    assert!(hs.contains(&1));
    assert!(hs.contains(&2));
}

#[test]
fn set_remove_if() {
    let mut set = BTreeEconomicSet::<i32>::create();
    set.add(1);
    set.add(2);
    set.add(3);
    set.add(4);
    let removed = set.remove_if(&|x| x % 2 == 0);
    assert!(removed);
    assert_eq!(set.size(), 2);
    assert!(set.contains(&1));
    assert!(set.contains(&3));
    assert!(!set.contains(&2));
    assert!(!set.contains(&4));
}

#[test]
fn set_create_from() {
    let mut src = BTreeEconomicSet::<i32>::create();
    src.add(1);
    src.add(2);
    let dst = BTreeEconomicSet::<i32>::create_from(&src);
    assert_eq!(dst.size(), 2);
    assert!(dst.contains(&1));
    assert!(dst.contains(&2));
}

#[test]
fn set_get_equivalence_strategy() {
    let set = BTreeEconomicSet::<i32>::create();
    assert_eq!(set.get_equivalence_strategy(), Equivalence::DEFAULT);

    let set = BTreeEconomicSet::<i32>::create_with_strategy(Equivalence::IDENTITY);
    assert_eq!(set.get_equivalence_strategy(), Equivalence::IDENTITY);
}

#[test]
fn set_to_array() {
    let mut set = BTreeEconomicSet::<i32>::create();
    set.add(1);
    set.add(2);
    set.add(3);
    let mut target = vec![0i32; 3];
    set.to_array(&mut target);
    assert_eq!(target, vec![1, 2, 3]);
}

#[test]
#[should_panic(expected = "UnsupportedOperationException")]
fn set_to_array_wrong_length_panics() {
    let mut set = BTreeEconomicSet::<i32>::create();
    set.add(1);
    let mut target = vec![0i32; 2];
    set.to_array(&mut target);
}
