/*
 * Copyright (c) 2011, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};

use super::default_graph_blocks::DefaultGraphBlocks;
use super::default_graph_types::DefaultGraphTypes;
use super::graph_blocks::GraphBlocks;
use super::graph_elements::GraphElements;
use super::graph_locations::GraphLocations;
use super::graph_protocol::*;
use super::graph_structure::GraphStructure;
use super::graph_types::GraphTypes;
use super::protocol_impl::ProtocolImpl;

/// Instance of output to dump information about compiler compilations.
///
/// Type parameters:
/// - `G`: the type of graph this instance handles
/// - `M`: the type of methods this instance handles
pub struct GraphOutput<G, N, C, P, B, M, F, S, SP, L> {
    printer: ProtocolImpl<G, N, C, P, B, M, F, S, SP, L>,
}

impl<G, N, C, P, B, M, F, S, SP, L> GraphOutput<G, N, C, P, B, M, F, S, SP, L>
where
    G: Clone + 'static,
    N: Clone + 'static,
    C: Clone + std::fmt::Display + 'static,
    P: Clone + 'static,
    B: Clone + 'static,
    M: Clone + std::fmt::Display + 'static,
    F: Clone + 'static,
    S: Clone + 'static,
    SP: Clone + 'static,
    L: Clone + 'static,
{
    /// Creates a new Builder to configure a future instance of `GraphOutput`.
    pub fn new_builder<C2: Clone + std::fmt::Display + 'static, P2: Clone + 'static>(
        structure: Box<dyn GraphStructure<G, N, C2, P2>>,
    ) -> Builder<G, N, M, C2, P2> {
        Builder::new(structure)
    }

    /// Begins a compilation group.
    pub fn begin_group(
        &mut self,
        for_graph: &G,
        name: &str,
        short_name: &str,
        method: Option<&M>,
        bci: i32,
        properties: &HashMap<String, Box<dyn Any>>,
    ) -> io::Result<()> {
        self.printer
            .begin_group(for_graph, name, short_name, method, bci, properties)
    }

    /// Prints a single graph.
    pub fn print(
        &mut self,
        graph: &G,
        properties: &HashMap<String, Box<dyn Any>>,
        id: i32,
        format: &str,
        args: &[Box<dyn Any>],
    ) -> io::Result<()> {
        self.printer.print(graph, properties, id, format, args)
    }

    /// Ends compilation group.
    pub fn end_group(&mut self) -> io::Result<()> {
        self.printer.end_group()
    }

    /// Closes the output. Flushes and closes the underlying channel.
    pub fn close(&mut self) -> io::Result<()> {
        self.printer.close()
    }

    /// Writes raw bytes into the output.
    pub fn write(&mut self, src: &[u8]) -> io::Result<()> {
        self.printer.write(src)
    }
}

/// Builder to configure and create an instance of `GraphOutput`.
pub struct Builder<G, N, M, C, P> {
    structure: Box<dyn GraphStructure<G, N, C, P>>,
    types: Box<dyn GraphTypes>,
    #[allow(dead_code)]
    blocks: Option<()>,
    #[allow(dead_code)]
    elements: Option<()>,
    #[allow(dead_code)]
    locations: Option<()>,
    major: i32,
    minor: i32,
    explicit_version_set: bool,
    embedded_graph_output: bool,
    properties: HashMap<String, Box<dyn Any>>,
    _phantom: std::marker::PhantomData<(M,)>,
}

impl<G, N, M, C, P> Builder<G, N, M, C, P>
where
    G: Clone + 'static,
    N: Clone + 'static,
    M: Clone + std::fmt::Display + 'static,
    C: Clone + std::fmt::Display + 'static,
    P: Clone + 'static,
{
    fn new(structure: Box<dyn GraphStructure<G, N, C, P>>) -> Self {
        Builder {
            structure,
            types: Box::new(DefaultGraphTypes),
            blocks: None,
            elements: None,
            locations: None,
            major: 0,
            minor: 0,
            explicit_version_set: false,
            embedded_graph_output: false,
            properties: HashMap::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Chooses which version of the protocol to use.
    pub fn protocol_version(mut self, major_version: i32, minor_version: i32) -> Self {
        assert!(major_version >= 1, "Major must be positive");
        assert!(minor_version >= 0, "Minor must not be negative");

        if !(self.explicit_version_set
            || (major_version == 0)
            || (major_version > self.major)
            || ((major_version == self.major) && (minor_version >= self.minor)))
        {
            panic!(
                "Cannot downgrade from minimum required version {}.{}",
                -self.major, self.minor
            );
        }
        self.major = major_version;
        self.minor = minor_version;
        self.explicit_version_set = true;
        self
    }

    fn require_version(&mut self, req_major: i32, req_minor: i32) {
        assert!(req_major >= 1, "Major must be positive");
        assert!(req_minor >= 0, "Minor must not be negative");
        if self.explicit_version_set {
            if self.major < req_major || (self.major == req_major && self.minor < req_minor) {
                panic!(
                    "Feature unsupported in version {}.{}",
                    self.major, self.minor
                );
            }
        } else if self.major < req_major {
            self.major = req_major;
            self.minor = req_minor;
        } else if self.major == req_major {
            self.minor = self.minor.max(req_minor);
        }
    }

    /// Sets the output as embedded.
    pub fn embedded(mut self, embedded: bool) -> Self {
        self.embedded_graph_output = embedded;
        self
    }

    /// Associates a different implementation of types.
    pub fn types(mut self, graph_types: Box<dyn GraphTypes>) -> Self {
        self.types = graph_types;
        self
    }

    /// Associates implementation of blocks.
    pub fn blocks<B2: Clone + 'static>(
        self,
        _graph_blocks: Box<dyn GraphBlocks<G, B2, N>>,
    ) -> Self {
        self
    }

    /// Associates implementation of graph elements.
    pub fn elements<E2, FP2, SP2>(
        self,
        _graph_elements: Box<dyn GraphElements<E2, FP2, (), SP2>>,
    ) -> Builder<G, N, E2, C, P> {
        Builder {
            structure: self.structure,
            types: self.types,
            blocks: None,
            elements: None,
            locations: None,
            major: self.major,
            minor: self.minor,
            explicit_version_set: self.explicit_version_set,
            embedded_graph_output: self.embedded_graph_output,
            properties: self.properties,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Associates implementation of graph elements and locations.
    pub fn elements_and_locations<E2, FP2, SP2, LP2>(
        self,
        _graph_elements: Box<dyn GraphElements<E2, FP2, (), SP2>>,
        _graph_locations: Box<dyn GraphLocations<E2, SP2, LP2>>,
    ) -> Builder<G, N, E2, C, P> {
        Builder {
            structure: self.structure,
            types: self.types,
            blocks: None,
            elements: None,
            locations: None,
            major: self.major,
            minor: self.minor,
            explicit_version_set: self.explicit_version_set,
            embedded_graph_output: self.embedded_graph_output,
            properties: self.properties,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Attaches metadata to the dump.
    pub fn attr(mut self, name: &str, value: Box<dyn Any>) -> Self {
        self.require_version(7, 0);
        self.properties.insert(name.to_string(), value);
        self
    }

    /// Creates a new `GraphOutput` to output to the provided write target.
    #[allow(clippy::type_complexity)]
    pub fn build<W: Write + 'static>(
        self,
        writer: W,
    ) -> io::Result<GraphOutput<G, N, C, P, (), M, (), (), (), ()>> {
        let m = if self.major == 0 {
            DEFAULT_MAJOR_VERSION
        } else {
            self.major
        };
        let n = if self.major == 0 {
            DEFAULT_MINOR_VERSION
        } else {
            self.minor
        };

        let writer = RefCell::new(writer);
        let mut printer = ProtocolImpl::new(
            m,
            n,
            self.embedded_graph_output,
            self.structure,
            self.types,
            Box::new(DefaultGraphBlocks),
            None,
            None,
            Box::new(move |data: &[u8]| writer.borrow_mut().write_all(data)),
        )?;

        if !self.properties.is_empty() {
            printer.start_document(&self.properties)?;
        }

        Ok(GraphOutput { printer })
    }
}
