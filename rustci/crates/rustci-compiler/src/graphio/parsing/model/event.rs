/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::sync::Arc;

/// Abstract base class for events with listeners.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.Event<L>`.
pub struct Event<L> {
    listeners: std::sync::Mutex<Vec<Arc<dyn Fn(&L) + Send + Sync>>>,
    fire_events: std::sync::atomic::AtomicBool,
    event_was_fired: std::sync::atomic::AtomicBool,
}

impl<L: 'static> Event<L> {
    pub fn new() -> Self {
        Event {
            listeners: std::sync::Mutex::new(Vec::new()),
            fire_events: std::sync::atomic::AtomicBool::new(true),
            event_was_fired: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Adds a listener. Mirrors `Event.addListener(L)`.
    pub fn add_listener<F>(&self, listener: F)
    where
        F: Fn(&L) + Send + Sync + 'static,
    {
        if let Ok(mut listeners) = self.listeners.lock() {
            listeners.push(Arc::new(listener));
        }
    }

    /// Removes all listeners. Mirrors `Event.removeListener(L)`.
    pub fn remove_all_listeners(&self) {
        if let Ok(mut listeners) = self.listeners.lock() {
            listeners.clear();
        }
    }

    /// Fires the event to all listeners. Mirrors `Event.fire()`.
    pub fn fire(&self) {
        if self.fire_events.load(std::sync::atomic::Ordering::Acquire) {
            let listeners: Vec<Arc<dyn Fn(&L) + Send + Sync>> = self
                .listeners
                .lock()
                .map(|l| l.clone())
                .unwrap_or_default();
            for listener in &listeners {
                listener(&self.get_source());
            }
        } else {
            self.event_was_fired
                .store(true, std::sync::atomic::Ordering::Release);
        }
    }

    /// Begins an atomic update. Mirrors `Event.beginAtomic()`.
    pub fn begin_atomic(&self) {
        self.fire_events
            .store(false, std::sync::atomic::Ordering::Release);
        self.event_was_fired
            .store(false, std::sync::atomic::Ordering::Release);
    }

    /// Ends an atomic update. Mirrors `Event.endAtomic()`.
    pub fn end_atomic(&self) {
        self.fire_events
            .store(true, std::sync::atomic::Ordering::Release);
        if self.event_was_fired.load(std::sync::atomic::Ordering::Acquire) {
            self.fire();
        }
    }

    /// Returns the source object for this event. Override in subclasses.
    fn get_source(&self) -> L {
        unimplemented!("Event::get_source must be overridden")
    }

    /// Returns a clone of all listeners. Used by ChangedEvent to fire events.
    pub(crate) fn get_listeners_clone(&self) -> Vec<Arc<dyn Fn(&L) + Send + Sync>> {
        if let Ok(listeners) = self.listeners.lock() {
            listeners.clone()
        } else {
            Vec::new()
        }
    }
}

impl<L: 'static> Default for Event<L> {
    fn default() -> Self {
        Event::new()
    }
}