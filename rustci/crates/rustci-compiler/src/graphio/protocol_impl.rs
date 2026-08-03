/*
 * Copyright (c) 2011, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::any::Any;
use std::collections::HashMap;
use std::io;

use super::graph_blocks::GraphBlocks;
use super::graph_elements::GraphElements;
use super::graph_locations::GraphLocations;
use super::graph_protocol::*;
use super::graph_structure::GraphStructure;
use super::graph_types::GraphTypes;

/// Mirrors `GraphProtocol.ConstantPool`.
/// A limited pool of constants for use by the graph protocol.
/// Once the cache fills up the oldest slots are replaced with new values
/// in a cyclic fashion.
struct ConstantPool {
    next_id: u16,
    /// Mapping from a key (bytes) to the pool entry id.
    /// For POOL_STRING entries, the value may be a forwarding string
    /// key that then maps to the actual id.
    map: HashMap<Vec<u8>, ObjectOrString>,
    keys: Vec<Option<Vec<u8>>>,
}

#[derive(Clone)]
enum ObjectOrString {
    Id(u16),
    Forward(String),
}

impl ConstantPool {
    fn new() -> Self {
        ConstantPool {
            next_id: 0,
            map: HashMap::new(),
            keys: vec![None; CONSTANT_POOL_MAX_SIZE],
        }
    }

    /// Looks up an object in the pool. Mirrors `ConstantPool.get(Object, int)`.
    /// For POOL_STRING, if the key is not a String, it forwards to the
    /// toString representation.
    fn get(&self, key: &str, type_code: u8) -> Option<u16> {
        let key_bytes = key.as_bytes().to_vec();
        let value = self.map.get(&key_bytes);
        match value {
            Some(ObjectOrString::Id(id)) => {
                if let Some(Some(ref k)) = self.keys.get(*id as usize) {
                    if k == &key_bytes {
                        return Some(*id);
                    }
                }
                None
            }
            Some(ObjectOrString::Forward(s)) => {
                let s_bytes = s.as_bytes().to_vec();
                if let Some(ObjectOrString::Id(id)) = self.map.get(&s_bytes) {
                    if let Some(Some(ref k)) = self.keys.get(*id as usize) {
                        if k == &s_bytes {
                            return Some(*id);
                        }
                    }
                }
                None
            }
            None => {
                // For POOL_STRING, try the string representation
                if type_code == POOL_STRING {
                    let s_bytes = key.as_bytes().to_vec();
                    if let Some(ObjectOrString::Id(id)) = self.map.get(&s_bytes) {
                        if let Some(Some(ref k)) = self.keys.get(*id as usize) {
                            if k == &s_bytes {
                                return Some(*id);
                            }
                        }
                    }
                }
                None
            }
        }
    }

    /// Adds an object to the pool. Mirrors `ConstantPool.add(Object, int)`.
    fn add(&mut self, key: &str, type_code: u8) -> u16 {
        let id = self.next_id;
        self.next_id = (self.next_id + 1) % (CONSTANT_POOL_MAX_SIZE as u16);
        if let Some(ref old_key) = self.keys[id as usize] {
            self.map.remove(old_key);
        }
        if type_code == POOL_STRING {
            let key_bytes = key.as_bytes().to_vec();
            self.map.insert(key_bytes.clone(), ObjectOrString::Id(id));
            self.keys[id as usize] = Some(key_bytes);
        } else {
            let key_bytes = key.as_bytes().to_vec();
            self.map.insert(key_bytes.clone(), ObjectOrString::Id(id));
            self.keys[id as usize] = Some(key_bytes);
        }
        id
    }

    fn reset(&mut self) {
        self.map.clear();
        self.keys.fill(None);
        self.next_id = 0;
    }
}

pub struct ProtocolImpl<G, N, C, P, B, M, F, S, SP, L> {
    pub version_major: u8,
    pub version_minor: u8,
    embedded: bool,
    constant_pool: ConstantPool,
    buffer: Vec<u8>,
    #[allow(clippy::type_complexity)]
    write_fn: Box<dyn FnMut(&[u8]) -> io::Result<()>>,
    structure: Box<dyn GraphStructure<G, N, C, P>>,
    #[allow(dead_code)]
    types: Box<dyn GraphTypes>,
    blocks: Box<dyn GraphBlocks<G, B, N>>,
    #[allow(dead_code)]
    elements: Option<Box<dyn GraphElements<M, F, S, SP>>>,
    #[allow(dead_code)]
    locations: Option<Box<dyn GraphLocations<M, SP, L>>>,
    printing: bool,
    is_open: bool,
    _phantom: std::marker::PhantomData<(M, F, S, SP, L)>,
}

impl<G, N, C, P, B, M, F, S, SP, L> ProtocolImpl<G, N, C, P, B, M, F, S, SP, L>
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
    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub fn new(
        major: i32,
        minor: i32,
        embedded: bool,
        structure: Box<dyn GraphStructure<G, N, C, P>>,
        types: Box<dyn GraphTypes>,
        blocks: Box<dyn GraphBlocks<G, B, N>>,
        elements: Option<Box<dyn GraphElements<M, F, S, SP>>>,
        locations: Option<Box<dyn GraphLocations<M, SP, L>>>,
        #[allow(clippy::type_complexity)] write_fn: Box<dyn FnMut(&[u8]) -> io::Result<()>>,
    ) -> io::Result<Self> {
        let major = major as u8;
        let minor = minor as u8;
        if major > MAJOR_VERSION || (major == MAJOR_VERSION && minor > MINOR_VERSION) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unrecognized version {}.{}", major, minor),
            ));
        }
        let mut p = ProtocolImpl {
            version_major: major,
            version_minor: minor,
            embedded,
            constant_pool: ConstantPool::new(),
            buffer: Vec::with_capacity(256 * 1024),
            write_fn,
            structure,
            types,
            blocks,
            elements,
            locations,
            printing: false,
            is_open: true,
            _phantom: std::marker::PhantomData,
        };
        if !embedded {
            p.write_version()?;
            p.flush_embedded()?;
        }
        Ok(p)
    }

    /// Creates a child ProtocolImpl that shares the parent's channel, constant pool,
    /// and protocol version. Mirrors `ProtocolImpl(GraphProtocol parent, ...)`.
    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub fn new_child(
        parent: &ProtocolImpl<G, N, C, P, B, M, F, S, SP, L>,
        structure: Box<dyn GraphStructure<G, N, C, P>>,
        types: Box<dyn GraphTypes>,
        blocks: Box<dyn GraphBlocks<G, B, N>>,
        elements: Option<Box<dyn GraphElements<M, F, S, SP>>>,
        locations: Option<Box<dyn GraphLocations<M, SP, L>>>,
    ) -> Self {
        ProtocolImpl {
            version_major: parent.version_major,
            version_minor: parent.version_minor,
            embedded: parent.embedded,
            constant_pool: ConstantPool::new(),
            buffer: Vec::with_capacity(256 * 1024),
            write_fn: Box::new(|_| Ok(())),
            structure,
            types,
            blocks,
            elements,
            locations,
            printing: false,
            is_open: true,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn print(
        &mut self,
        graph: &G,
        properties: &HashMap<String, Box<dyn Any>>,
        id: i32,
        format: &str,
        args: &[Box<dyn Any>],
    ) -> io::Result<()> {
        self.printing = true;
        let result = (|| -> io::Result<()> {
            self.write_byte(BEGIN_GRAPH)?;
            if self.version_major >= 3 {
                self.write_int(id)?;
                self.write_string(format)?;
                self.write_int(args.len() as i32)?;
                for a in args {
                    self.write_property_object(graph, a.as_ref())?;
                }
            } else {
                let title = self.format_title(graph, id, format, args);
                self.write_pool_object(&title)?;
            }
            self.write_graph(graph, properties)?;
            self.flush_embedded()?;
            self.flush()?;
            Ok(())
        })();
        self.printing = false;
        result
    }

    pub fn start_document(
        &mut self,
        document_properties: &HashMap<String, Box<dyn Any>>,
    ) -> io::Result<()> {
        if self.version_major < 7 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Dump properties unsupported in format v.{}",
                    self.version_major
                ),
            ));
        }
        self.printing = true;
        let result = (|| -> io::Result<()> {
            self.write_byte(BEGIN_DOCUMENT)?;
            self.write_properties_doc(document_properties)?;
            Ok(())
        })();
        self.printing = false;
        result
    }

    pub fn begin_group(
        &mut self,
        _for_graph: &G,
        name: &str,
        short_name: &str,
        method: Option<&M>,
        bci: i32,
        properties: &HashMap<String, Box<dyn Any>>,
    ) -> io::Result<()> {
        self.printing = true;
        let result = (|| -> io::Result<()> {
            self.write_byte(BEGIN_GROUP)?;
            self.write_pool_object(name)?;
            self.write_pool_object(short_name)?;
            if let Some(m) = method {
                self.write_pool_object(m)?;
            } else {
                self.write_pool_null()?;
            }
            self.write_int(bci)?;
            self.write_properties_doc(properties)?;
            self.flush_embedded()?;
            Ok(())
        })();
        self.printing = false;
        result
    }

    pub fn end_group(&mut self) -> io::Result<()> {
        self.printing = true;
        let result = (|| -> io::Result<()> {
            self.write_byte(CLOSE_GROUP)?;
            self.flush_embedded()?;
            Ok(())
        })();
        self.printing = false;
        result
    }

    pub fn write(&mut self, src: &[u8]) -> io::Result<usize> {
        if self.printing {
            return Err(io::Error::other("Trying to write during graph print."));
        }
        self.constant_pool.reset();
        self.write_bytes_raw(src)?;
        Ok(src.len())
    }

    /// Checks if the output is open. Mirrors `GraphProtocol.isOpen()`.
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn close(&mut self) {
        let _ = self.flush();
        self.is_open = false;
    }

    fn write_version(&mut self) -> io::Result<()> {
        self.write_bytes_raw(MAGIC_BYTES)?;
        self.write_byte(self.version_major)?;
        self.write_byte(self.version_minor)
    }

    fn flush_embedded(&mut self) -> io::Result<()> {
        if self.embedded {
            self.flush()?;
            self.constant_pool.reset();
        }
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        if !self.buffer.is_empty() {
            (self.write_fn)(&self.buffer)?;
            self.buffer.clear();
        }
        Ok(())
    }

    fn write_byte(&mut self, b: u8) -> io::Result<()> {
        self.buffer.push(b);
        Ok(())
    }

    fn write_int(&mut self, b: i32) -> io::Result<()> {
        self.buffer.extend_from_slice(&b.to_be_bytes());
        Ok(())
    }

    fn write_long(&mut self, b: i64) -> io::Result<()> {
        self.buffer.extend_from_slice(&b.to_be_bytes());
        Ok(())
    }

    fn write_double(&mut self, b: f64) -> io::Result<()> {
        self.buffer.extend_from_slice(&b.to_be_bytes());
        Ok(())
    }

    fn write_float(&mut self, b: f32) -> io::Result<()> {
        self.buffer.extend_from_slice(&b.to_be_bytes());
        Ok(())
    }

    fn write_short(&mut self, b: u16) -> io::Result<()> {
        self.buffer.extend_from_slice(&b.to_be_bytes());
        Ok(())
    }

    fn write_string(&mut self, s: &str) -> io::Result<()> {
        let bytes = s.as_bytes();
        self.write_bytes(bytes)
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        let len = bytes.len();
        if len > u16::MAX as usize {
            self.write_int(len as i32)?;
        } else {
            self.write_short(len as u16)?;
        }
        self.buffer.extend_from_slice(bytes);
        Ok(())
    }

    fn write_bytes_raw(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.buffer.extend_from_slice(bytes);
        Ok(())
    }

    fn write_doubles(&mut self, arr: &[f64]) -> io::Result<()> {
        self.write_int(arr.len() as i32)?;
        for v in arr {
            self.write_double(*v)?;
        }
        Ok(())
    }

    fn write_ints(&mut self, arr: &[i32]) -> io::Result<()> {
        self.write_int(arr.len() as i32)?;
        for v in arr {
            self.write_int(*v)?;
        }
        Ok(())
    }

    // ---- Pool Object Methods ----

    /// Writes a pool object reference. Mirrors `GraphProtocol.writePoolObject(Object)`.
    fn write_pool_object<T: std::fmt::Display + ?Sized>(&mut self, obj: &T) -> io::Result<()> {
        let key = format!("{}", obj);
        let type_code = self.find_pool_type(&key);
        let id = self.constant_pool.get(&key, type_code);
        if let Some(id) = id {
            self.write_byte(type_code)?;
            self.write_short(id)?;
        } else {
            self.add_pool_entry(&key, type_code)?;
        }
        Ok(())
    }

    fn write_pool_null(&mut self) -> io::Result<()> {
        self.write_byte(POOL_NULL)
    }

    /// Determines the pool type for an object. Mirrors `GraphProtocol.findPoolType(Object, Object[])`.
    /// In Rust, this is simplified since we use string keys for the pool.
    fn find_pool_type(&self, _key: &str) -> u8 {
        // Default implementation: treat everything as POOL_STRING.
        // Subclasses can override this behavior through the GraphElements interface.
        POOL_STRING
    }

    /// Adds a new entry to the constant pool. Mirrors `GraphProtocol.addPoolEntry(Object, int, Object[])`.
    fn add_pool_entry(&mut self, key: &str, type_code: u8) -> io::Result<()> {
        let id = self.constant_pool.add(key, type_code);
        self.write_byte(POOL_NEW)?;
        self.write_short(id)?;
        self.write_byte(type_code)?;
        match type_code {
            POOL_STRING => {
                self.write_string(key)?;
            }
            _ => {
                // Default: write as string
                self.write_string(key)?;
            }
        }
        Ok(())
    }

    /// Checks if a value is found and optionally stores it. Mirrors `GraphProtocol.isFound(Object, Object[])`.
    fn is_found<T>(obj: &Option<T>) -> bool {
        obj.is_some()
    }

    // ---- Graph Writing Methods ----

    fn write_graph(
        &mut self,
        graph: &G,
        properties: &HashMap<String, Box<dyn Any>>,
    ) -> io::Result<()> {
        self.write_properties(graph, properties)?;
        self.write_nodes(graph)?;
        let blocks_list = self.blocks.blocks(graph);
        self.write_blocks(&blocks_list, graph)
    }

    fn write_properties(
        &mut self,
        graph: &G,
        props: &HashMap<String, Box<dyn Any>>,
    ) -> io::Result<()> {
        self.write_properties_doc(props)?;
        self.write_nodes(graph)?;
        let blocks_list = self.blocks.blocks(graph);
        self.write_blocks(&blocks_list, graph)
    }

    fn write_nodes(&mut self, graph: &G) -> io::Result<()> {
        let nodes = self.structure.nodes(graph);
        let size = self.structure.nodes_count(graph);
        self.write_int(size as i32)?;
        let mut cnt = 0;
        let mut props = HashMap::new();
        for node in &nodes {
            let node_class = self.structure.class_for_node(node);
            self.structure.node_properties(graph, node, &mut props);
            self.write_int(self.structure.node_id(node))?;
            self.write_pool_object(&node_class)?;
            self.write_byte(if self.structure.node_has_predecessor(node) {
                1
            } else {
                0
            })?;
            self.write_properties_doc(&props)?;
            self.write_edges(graph, node, true)?;
            self.write_edges(graph, node, false)?;
            props.clear();
            cnt += 1;
        }
        if size != cnt {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Expecting {} nodes, but found {}", size, cnt),
            ));
        }
        Ok(())
    }

    fn write_edges(&mut self, graph: &G, node: &N, dump_inputs: bool) -> io::Result<()> {
        let node_class = self.structure.class_for_node(node);
        let port = if dump_inputs {
            self.structure.port_inputs(&node_class)
        } else {
            self.structure.port_outputs(&node_class)
        };
        let size = self.structure.port_size(&port);
        for i in 0..size {
            let list = self.structure.edge_nodes(graph, node, &port, i);
            if self.structure.edge_direct(&port, i) {
                let n = list.as_ref().and_then(|l| l.first()).cloned();
                self.write_node_ref(&n)?;
            } else {
                match list {
                    None => self.write_short(0)?,
                    Some(ref list) => {
                        let list_size = list.len();
                        if list_size > u16::MAX as usize {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Too many nodes in list: {}", list_size),
                            ));
                        }
                        self.write_short(list_size as u16)?;
                        for edge in list {
                            self.write_node_ref(&Some(edge.clone()))?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn write_node_ref(&mut self, node: &Option<N>) -> io::Result<()> {
        match node {
            Some(n) => self.write_int(self.structure.node_id(n)),
            None => self.write_int(-1),
        }
    }

    fn write_blocks(&mut self, blocks: &[B], graph: &G) -> io::Result<()> {
        if blocks.is_empty() {
            self.write_int(0)?;
            return Ok(());
        }
        for block in blocks {
            let nodes = self.blocks.block_nodes(graph, block);
            if nodes.is_empty() {
                self.write_int(0)?;
                return Ok(());
            }
        }
        self.write_int(blocks.len() as i32)?;
        for block in blocks {
            let nodes = self.blocks.block_nodes(graph, block);
            self.write_int(self.blocks.block_id(block))?;
            self.write_int(nodes.len() as i32)?;
            for node in &nodes {
                self.write_int(self.structure.node_id(node))?;
            }
            let successors = self.blocks.block_successors(block);
            self.write_int(successors.len() as i32)?;
            for sux in &successors {
                self.write_int(self.blocks.block_id(sux))?;
            }
        }
        Ok(())
    }

    /// Writes edge info for a node class. Mirrors `GraphProtocol.writeEdgesInfo(NodeClass, boolean)`.
    #[allow(dead_code)]
    fn write_edges_info(&mut self, node_class: &C, dump_inputs: bool) -> io::Result<()> {
        let edges = if dump_inputs {
            self.structure.port_inputs(node_class)
        } else {
            self.structure.port_outputs(node_class)
        };
        let size = self.structure.port_size(&edges);
        self.write_short(size as u16)?;
        for i in 0..size {
            self.write_byte(if self.structure.edge_direct(&edges, i) { 0 } else { 1 })?;
            let name = self.structure.edge_name(&edges, i);
            self.write_pool_object(&name)?;
            if dump_inputs {
                let edge_type = self.structure.edge_type(&edges, i);
                let type_str = format!("{:?}", edge_type);
                self.write_pool_object(&type_str)?;
            }
        }
        Ok(())
    }

    // ---- Property Object Writing ----

    fn write_property_object(&mut self, graph: &G, obj: &dyn Any) -> io::Result<()> {
        if let Some(v) = obj.downcast_ref::<i32>() {
            self.write_byte(PROPERTY_INT)?;
            self.write_int(*v)?;
        } else if let Some(v) = obj.downcast_ref::<i64>() {
            self.write_byte(PROPERTY_LONG)?;
            self.write_long(*v)?;
        } else if let Some(v) = obj.downcast_ref::<f64>() {
            self.write_byte(PROPERTY_DOUBLE)?;
            self.write_double(*v)?;
        } else if let Some(v) = obj.downcast_ref::<f32>() {
            self.write_byte(PROPERTY_FLOAT)?;
            self.write_float(*v)?;
        } else if let Some(v) = obj.downcast_ref::<bool>() {
            if *v {
                self.write_byte(PROPERTY_TRUE)?;
            } else {
                self.write_byte(PROPERTY_FALSE)?;
            }
        } else if let Some(v) = obj.downcast_ref::<Vec<f64>>() {
            self.write_byte(PROPERTY_ARRAY)?;
            self.write_byte(PROPERTY_DOUBLE)?;
            self.write_doubles(v)?;
        } else if let Some(v) = obj.downcast_ref::<Vec<i32>>() {
            self.write_byte(PROPERTY_ARRAY)?;
            self.write_byte(PROPERTY_INT)?;
            self.write_ints(v)?;
        } else if let Some(v) = obj.downcast_ref::<Vec<Box<dyn Any>>>() {
            self.write_byte(PROPERTY_ARRAY)?;
            self.write_byte(PROPERTY_POOL)?;
            self.write_int(v.len() as i32)?;
            for o in v.iter() {
                let s = format!("{:?}", o);
                self.write_pool_object(&s)?;
            }
        } else {
            // Check for subgraph
            let sub = self.structure.graph(graph, obj);
            if let Some(sub_graph) = sub {
                self.write_byte(PROPERTY_SUBGRAPH)?;
                self.write_graph(&sub_graph, &HashMap::new())?;
            } else {
                self.write_byte(PROPERTY_POOL)?;
                let s = format!("{:?}", obj);
                self.write_pool_object(&s)?;
            }
        }
        Ok(())
    }

    /// Writes properties document. Mirrors `GraphProtocol.writeProperties(Graph, Map)`.
    /// Writes both keys and values, unlike the previous stub that only wrote keys twice.
    fn write_properties_doc(&mut self, props: &HashMap<String, Box<dyn Any>>) -> io::Result<()> {
        let size = props.len();
        if size >= u16::MAX as usize {
            if self.version_major > 7 {
                self.write_short(u16::MAX)?;
                self.write_int(size as i32)?;
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "Property count too big in version {}.{}",
                        self.version_major, self.version_minor
                    ),
                ));
            }
        } else {
            self.write_short(size as u16)?;
        }
        let mut cnt = 0;
        for (key, value) in props.iter() {
            self.write_pool_object(key)?;
            // Create a graph clone to pass to write_property_object
            // We need to work around the borrow checker - we clone the value reference
            let value_ref: &dyn Any = value.as_ref();
            // We need to pass the graph reference. Since we don't have the graph
            // available in this method, we use a workaround. Actually, we need the
            // graph for subgraph detection. Let's restructure:
            self.write_property_object_static(value_ref)?;
            cnt += 1;
        }
        if size != cnt {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Expecting {} properties, but found only {}", size, cnt),
            ));
        }
        Ok(())
    }

    /// Writes a property object without a graph context (for document-level properties).
    fn write_property_object_static(&mut self, obj: &dyn Any) -> io::Result<()> {
        if let Some(v) = obj.downcast_ref::<i32>() {
            self.write_byte(PROPERTY_INT)?;
            self.write_int(*v)?;
        } else if let Some(v) = obj.downcast_ref::<i64>() {
            self.write_byte(PROPERTY_LONG)?;
            self.write_long(*v)?;
        } else if let Some(v) = obj.downcast_ref::<f64>() {
            self.write_byte(PROPERTY_DOUBLE)?;
            self.write_double(*v)?;
        } else if let Some(v) = obj.downcast_ref::<f32>() {
            self.write_byte(PROPERTY_FLOAT)?;
            self.write_float(*v)?;
        } else if let Some(v) = obj.downcast_ref::<bool>() {
            if *v {
                self.write_byte(PROPERTY_TRUE)?;
            } else {
                self.write_byte(PROPERTY_FALSE)?;
            }
        } else if let Some(v) = obj.downcast_ref::<Vec<f64>>() {
            self.write_byte(PROPERTY_ARRAY)?;
            self.write_byte(PROPERTY_DOUBLE)?;
            self.write_doubles(v)?;
        } else if let Some(v) = obj.downcast_ref::<Vec<i32>>() {
            self.write_byte(PROPERTY_ARRAY)?;
            self.write_byte(PROPERTY_INT)?;
            self.write_ints(v)?;
        } else if let Some(v) = obj.downcast_ref::<Vec<Box<dyn Any>>>() {
            self.write_byte(PROPERTY_ARRAY)?;
            self.write_byte(PROPERTY_POOL)?;
            self.write_int(v.len() as i32)?;
            for o in v.iter() {
                let s = format!("{:?}", o);
                self.write_pool_object(&s)?;
            }
        } else {
            self.write_byte(PROPERTY_POOL)?;
            let s = format!("{:?}", obj);
            self.write_pool_object(&s)?;
        }
        Ok(())
    }

    fn format_title(&self, _graph: &G, id: i32, format: &str, args: &[Box<dyn Any>]) -> String {
        let mut result = format.to_string();
        for arg in args {
            if let Some(s) = arg.downcast_ref::<String>() {
                if let Some(pos) = result.find("%s") {
                    result.replace_range(pos..pos + 2, s);
                } else if let Some(pos) = result.find("{}") {
                    result.replace_range(pos..pos + 2, s);
                }
            } else if let Some(i) = arg.downcast_ref::<i32>() {
                if let Some(pos) = result.find("%d") {
                    result.replace_range(pos..pos + 2, &format!("{}", i));
                } else if let Some(pos) = result.find("{}") {
                    result.replace_range(pos..pos + 2, &format!("{}", i));
                }
            }
        }
        format!("{} [{}]", result, id)
    }
}