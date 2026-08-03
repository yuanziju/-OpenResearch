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

struct ConstantPool {
    next_id: u16,
    keys: Vec<Vec<u8>>,
    map: HashMap<Vec<u8>, u16>,
}

impl ConstantPool {
    fn new() -> Self {
        ConstantPool {
            next_id: 0,
            keys: vec![Vec::new(); CONSTANT_POOL_MAX_SIZE],
            map: HashMap::new(),
        }
    }

    fn get(&self, key: &[u8]) -> Option<u16> {
        self.map.get(key).copied()
    }

    fn add(&mut self, key: Vec<u8>) -> u16 {
        let id = self.next_id;
        self.next_id = (self.next_id + 1) % (CONSTANT_POOL_MAX_SIZE as u16);
        if !self.keys[id as usize].is_empty() {
            let old_key = self.keys[id as usize].clone();
            self.map.remove(&old_key);
        }
        self.map.insert(key.clone(), id);
        self.keys[id as usize] = key;
        id
    }

    fn reset(&mut self) {
        self.map.clear();
        self.keys.fill(Vec::new());
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
            _phantom: std::marker::PhantomData,
        };
        if !embedded {
            p.write_version()?;
            p.flush_embedded()?;
        }
        Ok(p)
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

    pub fn write(&mut self, src: &[u8]) -> io::Result<()> {
        if self.printing {
            return Err(io::Error::other("Trying to write during graph print."));
        }
        self.constant_pool.reset();
        self.write_bytes_raw(src)
    }

    pub fn close(&mut self) -> io::Result<()> {
        self.flush()
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

    fn write_pool_object<T: std::fmt::Display + ?Sized>(&mut self, obj: &T) -> io::Result<()> {
        let key = format!("{}", obj).into_bytes();
        if let Some(id) = self.constant_pool.get(&key) {
            self.write_byte(POOL_STRING)?;
            self.write_short(id)?;
        } else {
            let id = self.constant_pool.add(key.clone());
            self.write_byte(POOL_NEW)?;
            self.write_short(id)?;
            self.write_byte(POOL_STRING)?;
            self.write_string(&String::from_utf8_lossy(&key))?;
        }
        Ok(())
    }

    fn write_pool_null(&mut self) -> io::Result<()> {
        self.write_byte(POOL_NULL)
    }

    fn write_graph(
        &mut self,
        graph: &G,
        properties: &HashMap<String, Box<dyn Any>>,
    ) -> io::Result<()> {
        self.write_properties(graph, properties)
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

    fn write_property_object(&mut self, _graph: &G, obj: &dyn Any) -> io::Result<()> {
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
        } else {
            self.write_byte(PROPERTY_POOL)?;
            let s = format!("{:?}", obj);
            self.write_pool_object(&s)?;
        }
        Ok(())
    }

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
        for key in props.keys() {
            self.write_pool_object(key)?;
            self.write_byte(PROPERTY_POOL)?;
            self.write_pool_object(key)?;
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
