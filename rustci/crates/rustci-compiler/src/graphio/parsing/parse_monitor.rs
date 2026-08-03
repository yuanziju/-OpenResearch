/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Monitor for parsing progress and cancellation.
pub trait ParseMonitor {
    /// Tick to report progress.
    fn update_progress(&self);

    /// Provides state detail information.
    fn set_state(&self, state: &str);

    /// Determines if the work was cancelled.
    fn is_cancelled(&self) -> bool;

    /// Reports a loading error.
    fn report_error(&self, parent_names: Vec<String>, name: String, error_message: String);
}
