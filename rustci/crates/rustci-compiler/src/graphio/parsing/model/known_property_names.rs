/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Well-known property names used in graph I/O.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.KnownPropertyNames`.
pub struct KnownPropertyNames;

impl KnownPropertyNames {
    pub const PROPNAME_HAS_PREDECESSOR: &'static str = "hasPredecessor";
    pub const PROPNAME_IDX: &'static str = "idx";
    pub const PROPNAME_SHORT_NAME: &'static str = "shortName";
    pub const PROPNAME_NAME: &'static str = "name";
    pub const PROPNAME_CLASS: &'static str = "class";
    pub const PROPNAME_BLOCK: &'static str = "block";
    pub const PROPNAME_STATE: &'static str = "state";
    pub const PROPNAME_NODE_SOURCE_POSITION: &'static str = "nodeSourcePosition";
    pub const PROPNAME_DUPLICATE: &'static str = "_isDuplicate";
    pub const PROPNAME_TYPE: &'static str = "type";
    pub const PROPNAME_ID: &'static str = "id";
    pub const PROPNAME_FREQUENCY: &'static str = "relativeFrequency";
    pub const PROPNAME_EXCEPTION_PROBABILITY: &'static str = "exceptionProbability";
    pub const PROPNAME_TRUE_PROBABILITY: &'static str = "trueSuccessorProbability";
    pub const PROPNAME_DUMP_SPEC: &'static str = "dump_spec";
    pub const PROPNAME_FIGURE: &'static str = "figure";
    pub const PROPNAME_CONNECTION_COUNT: &'static str = "connectionCount";
    pub const PROPNAME_PREDECESSOR_COUNT: &'static str = "predecessorCount";
    pub const PROPNAME_VM_UUID: &'static str = "vm.uuid";
    pub const PROPNAME_CMDLINE: &'static str = "sun.java.command";
    pub const PROPNAME_JVM_ARGS: &'static str = "jvmArguments";
    pub const PROPNAME_USER_LABEL: &'static str = "igv.userLabel";
}