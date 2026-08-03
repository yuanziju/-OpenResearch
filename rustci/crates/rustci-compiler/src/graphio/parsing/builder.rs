/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::any::Any;
use std::sync::Arc;

use super::constant_pool::ConstantPool;

/// Interface for building graph model data from the binary stream.
pub trait Builder {
    fn set_model_control(&mut self, ctrl: Box<dyn ModelControl>);

    fn add_block_edge(&mut self, from: i32, to: i32);

    fn add_node_to_block(&mut self, node_id: i32);

    fn end(&mut self);

    fn end_block(&mut self, id: i32);

    fn end_graph(&mut self);

    fn end_group(&mut self);

    fn end_node(&mut self, node_id: i32);

    fn constant_pool(&self) -> &ConstantPool;

    fn constant_pool_mut(&mut self) -> &mut ConstantPool;

    fn node_properties(&self, node_id: i32) -> &dyn Any;

    fn input_edge(&mut self, p: &Port, from: i32, to: i32, num: u16, index: i32);

    fn make_block_edges(&mut self);

    fn make_graph_edges(&mut self);

    fn mark_graph_duplicate(&mut self);

    fn reset_stream_data(&mut self);

    fn set_group_name(&mut self, name: &str, short_name: &str);

    fn set_method(&mut self, name: &str, short_name: &str, bci: i32);

    fn set_node_name(&mut self, node_class: &NodeClass);

    fn set_node_property(&mut self, key: &str, value: Arc<dyn Any + Send + Sync>);

    fn set_property(&mut self, key: &str, value: Arc<dyn Any + Send + Sync>);

    fn set_property_size(&mut self, size: i32);

    fn start(&mut self);

    fn start_block(&mut self, id: i32);

    fn start_block_by_name(&mut self, name: &str);

    fn start_graph(&mut self, dump_id: i32, format: &str, args: &[Arc<dyn Any + Send + Sync>]);

    fn start_graph_contents(&mut self);

    fn start_group(&mut self);

    fn start_group_content(&mut self);

    fn start_nested_property(&mut self, property_key: &str);

    fn start_node(&mut self, node_id: i32, has_predecessors: bool, node_class: &NodeClass);

    fn start_root(&mut self);

    fn successor_edge(&mut self, p: &Port, from: i32, to: i32, num: u16, index: i32);

    fn start_document_header(&mut self);

    fn end_document_header(&mut self);

    fn graph_content_digest(&mut self, dg: &[u8]);

    fn report_loading_error(&mut self, log_message: &str, parent_names: &[String]);
}

/// Control interface for the constant pool during model building.
pub trait ModelControl {
    fn constant_pool(&self) -> &ConstantPool;

    fn set_constant_pool(&mut self, c: ConstantPool);
}

/// Length classification for graph elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Length {
    S,
    M,
    L,
}

/// A port in a node class.
#[derive(Debug, Clone)]
pub struct Port {
    pub is_list: bool,
    pub name: String,
    pub ids: Vec<i32>,
}

impl Port {
    pub fn new(is_list: bool, name: String) -> Self {
        Port {
            is_list,
            name,
            ids: Vec::new(),
        }
    }
}

impl PartialEq for Port {
    fn eq(&self, other: &Self) -> bool {
        self.is_list == other.is_list && self.name == other.name
    }
}

impl Eq for Port {}

impl std::hash::Hash for Port {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.is_list.hash(state);
        self.name.hash(state);
    }
}

/// A typed port that includes an enum value for the edge type.
#[derive(Debug, Clone)]
pub struct TypedPort {
    pub port: Port,
    pub type_value: Option<Arc<dyn Any + Send + Sync>>,
}

impl TypedPort {
    pub fn new(
        is_list: bool,
        name: String,
        type_value: Option<Arc<dyn Any + Send + Sync>>,
    ) -> Self {
        TypedPort {
            port: Port::new(is_list, name),
            type_value,
        }
    }
}

/// A node in the graph model.
#[derive(Debug, Clone)]
pub struct Node {
    pub id: i32,
    pub class: NodeClass,
}

/// A node class in the graph model.
#[derive(Debug, Clone)]
pub struct NodeClass {
    pub class_name: String,
    pub name_template: String,
    pub inputs: Vec<TypedPort>,
    pub sux: Vec<Port>,
}

impl NodeClass {
    pub fn new(
        class_name: String,
        name_template: String,
        inputs: Vec<TypedPort>,
        sux: Vec<Port>,
    ) -> Self {
        NodeClass {
            class_name,
            name_template,
            inputs,
            sux,
        }
    }

    pub fn short_name(&self) -> String {
        if let Some(last_dot) = self.class_name.rfind('.') {
            let local = &self.class_name[last_dot + 1..];
            if local.ends_with("Node") && local != "StartNode" && local != "EndNode" {
                local[..local.len() - 4].to_string()
            } else {
                local.to_string()
            }
        } else {
            self.class_name.clone()
        }
    }
}

impl PartialEq for NodeClass {
    fn eq(&self, other: &Self) -> bool {
        self.class_name == other.class_name
            && self.inputs.len() == other.inputs.len()
            && self.sux.len() == other.sux.len()
    }
}

impl std::fmt::Display for NodeClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.class_name)
    }
}
