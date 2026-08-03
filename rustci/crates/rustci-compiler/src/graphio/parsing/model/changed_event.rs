/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Class representing a generic changed event.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.ChangedEvent<T>`.
///
/// In Java, the object is stored as a direct reference to `this`. In Rust,
/// we use a raw pointer to avoid self-referential struct issues. The pointer
/// must be set via `set_object()` before `fire()` is called and must remain
/// valid for the lifetime of this event.
pub struct ChangedEvent<T> {
    /// The object for which the event fires. Raw pointer to avoid self-referential issues.
    object: *const T,
    event: super::event::Event<T>,
}

// ChangedEvent is Send+Sync if T is Send+Sync, since we only dereference
// the pointer immutably in fire().
unsafe impl<T: Send + Sync> Send for ChangedEvent<T> {}
unsafe impl<T: Send + Sync> Sync for ChangedEvent<T> {}

impl<T: Send + Sync + 'static> ChangedEvent<T> {
    /// Creates a new event with the specific object as the one for which the event gets fired.
    /// Mirrors `ChangedEvent(T object)`.
    pub fn new(object: &T) -> Self {
        ChangedEvent {
            object: object as *const T,
            event: super::event::Event::new(),
        }
    }

    /// Creates a new event with no object set. The object must be set via
    /// `set_object()` before `fire()` is called.
    pub fn new_empty() -> Self {
        ChangedEvent {
            object: std::ptr::null(),
            event: super::event::Event::new(),
        }
    }

    /// Sets the object for this event. Must be called before `fire()` if
    /// the event was created with `new_empty()`. The pointer must remain
    /// valid for the lifetime of this event.
    pub fn set_object(&mut self, object: &T) {
        self.object = object as *const T;
    }

    /// Adds a listener. Mirrors `Event.addListener(L)`.
    pub fn add_listener<F>(&self, listener: F)
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        self.event.add_listener(listener);
    }

    /// Removes all listeners.
    pub fn remove_all_listeners(&self) {
        self.event.remove_all_listeners();
    }

    /// Fires the event to all listeners. Mirrors `ChangedEvent.fire()`.
    pub fn fire(&self) {
        if self.object.is_null() {
            return;
        }
        // SAFETY: The object pointer was set by `set_object()` or `new()`
        // and must remain valid for the lifetime of this event.
        let object = unsafe { &*self.object };
        let listeners = self.event.get_listeners_clone();
        for listener in &listeners {
            listener(object);
        }
    }

    /// Begins an atomic update. Mirrors `Event.beginAtomic()`.
    pub fn begin_atomic(&self) {
        self.event.begin_atomic();
    }

    /// Ends an atomic update. Mirrors `Event.endAtomic()`.
    pub fn end_atomic(&self) {
        self.event.end_atomic();
    }
}

impl<T: Send + Sync + 'static> super::changed_event_provider::ChangedEventProvider<T>
    for ChangedEvent<T>
{
    fn get_changed_event(&self) -> &ChangedEvent<T> {
        self
    }
}