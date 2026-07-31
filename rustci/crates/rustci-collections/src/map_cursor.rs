// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception
//
// Port of `org.graalvm.collections.UnmodifiableMapCursor<K, V>` and
// `org.graalvm.collections.MapCursor<K, V>`.
//
// `MapCursor extends UnmodifiableMapCursor`. The base trait exposes
// `advance` / `getKey` / `getValue`; the mutable subtrait adds `remove`
// (abstract) and `setValue` (default `UnsupportedOperationException`).
//
// Null-value semantics: Java's `getValue()` may return `null` for an entry
// whose value is null. In Rust the value type `V` is the non-null carrier and
// null is represented by `Option<V>` throughout the collections crate, so
// `get_value` returns `Option<&V>` (`None` for a null-value entry). Mirrors
// Java returning `null` from `getValue()` for such entries.

/// Cursor to iterate over a map without changing its contents. Mirrors
/// `org.graalvm.collections.UnmodifiableMapCursor<K, V>`.
pub trait UnmodifiableMapCursor<K, V> {
    /// Advances to the next entry. Returns `true` if a next entry exists,
    /// `false` if there is no next entry. Mirrors `advance()`.
    fn advance(&mut self) -> bool;

    /// The key of the current entry. Panics (mirroring Java
    /// `NoSuchElementException`) if no entry is current. Mirrors `getKey()`.
    fn get_key(&self) -> &K;

    /// The value of the current entry. Returns `None` if the current entry's
    /// value is null. Panics (mirroring Java `NoSuchElementException`) if no
    /// entry is current. Mirrors `getValue()`.
    fn get_value(&self) -> Option<&V>;
}

/// Cursor to iterate over a mutable map. Mirrors
/// `org.graalvm.collections.MapCursor<K, V>`.
pub trait MapCursor<K, V>: UnmodifiableMapCursor<K, V> {
    /// Remove the current entry from the map. May only be called once. After
    /// calling `remove`, it is no longer valid to call `get_key` or
    /// `get_value` on the current entry. Mirrors `remove()`.
    fn remove(&mut self);

    /// Set the value of the current entry. Returns the previous value
    /// associated with the current key (`None` if it was null). Default
    /// implementation throws `UnsupportedOperationException`, matching Java.
    fn set_value(&mut self, _new_value: Option<V>) -> Option<V> {
        panic!("UnsupportedOperationException: setValue not supported");
    }
}
