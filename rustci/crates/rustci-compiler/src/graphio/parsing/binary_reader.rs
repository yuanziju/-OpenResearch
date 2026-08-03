/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::any::Any;
use std::io;
use std::sync::Arc;

use super::binary_stream_defs::*;
use super::builder::{Builder as ModelBuilder, NodeClass, TypedPort};
use super::constant_pool::ConstantPool;
use super::data_source::DataSource;
use super::name_translator::NameTranslator;

/// Reads binary graph data from a DataSource and builds graph model objects.
pub struct BinaryReader<S: DataSource> {
    source: S,
    constant_pool: ConstantPool,
    name_translator: Option<Box<dyn NameTranslator>>,
    builder: Option<Box<dyn ModelBuilder>>,
    major_version: u8,
    minor_version: u8,
}

/// Represents a Java class in the pool.
#[derive(Debug, Clone)]
pub struct Klass {
    pub name: String,
}

/// Represents an enum class in the pool.
#[derive(Debug, Clone)]
pub struct EnumKlass {
    pub name: String,
    pub values: Vec<String>,
}

/// Represents an enum value in the pool.
#[derive(Debug, Clone)]
pub struct EnumValue {
    pub enum_klass: EnumKlass,
    pub ordinal: i32,
}

/// Represents a Java method in the pool.
#[derive(Debug, Clone)]
pub struct Method {
    pub holder: Klass,
    pub name: String,
    pub signature: Signature,
    pub access_flags: i32,
    pub code: Vec<u8>,
}

/// Represents a Java field in the pool.
#[derive(Debug, Clone)]
pub struct Field {
    pub holder: Klass,
    pub name: String,
    pub field_type: String,
    pub access_flags: i32,
}

/// Represents a method signature in the pool.
#[derive(Debug, Clone)]
pub struct Signature {
    pub arg_types: Vec<String>,
    pub return_type: String,
}

impl Signature {
    pub fn new(arg_types: Vec<String>, return_type: String) -> Self {
        Signature {
            arg_types,
            return_type,
        }
    }
}

impl<S: DataSource> BinaryReader<S> {
    pub fn new(source: S) -> Self {
        BinaryReader {
            source,
            constant_pool: ConstantPool::new(),
            name_translator: None,
            builder: None,
            major_version: 0,
            minor_version: 0,
        }
    }

    pub fn with_name_translator(mut self, translator: Box<dyn NameTranslator>) -> Self {
        self.name_translator = Some(translator);
        self
    }

    pub fn with_builder(mut self, builder: Box<dyn ModelBuilder>) -> Self {
        self.builder = Some(builder);
        self
    }

    pub fn set_builder(&mut self, builder: Box<dyn ModelBuilder>) {
        self.builder = Some(builder);
    }

    pub fn source(&self) -> &S {
        &self.source
    }

    pub fn source_mut(&mut self) -> &mut S {
        &mut self.source
    }

    pub fn constant_pool(&self) -> &ConstantPool {
        &self.constant_pool
    }

    pub fn constant_pool_mut(&mut self) -> &mut ConstantPool {
        &mut self.constant_pool
    }

    pub fn major_version(&self) -> u8 {
        self.major_version
    }

    pub fn minor_version(&self) -> u8 {
        self.minor_version
    }

    /// Translates a class name using the registered translator, if any.
    pub fn translate(&self, fqn: &str) -> String {
        if let Some(ref t) = self.name_translator {
            t.translate(fqn)
        } else {
            fqn.to_string()
        }
    }

    /// Reads the file header and validates format.
    pub fn read_header(&mut self) -> io::Result<bool> {
        self.source.read_header()
    }

    /// Reads a pool object from the stream.
    pub fn read_pool_object(&mut self) -> io::Result<Arc<dyn Any + Send + Sync>> {
        let pool_type = self.source.read_byte()?;
        match pool_type {
            POOL_NEW => {
                let index = self.source.read_short()? as usize;
                let obj_type = self.source.read_byte()?;
                let obj = self.read_pool_entry(obj_type)?;
                self.constant_pool.add_pool_entry(index, obj.clone());
                Ok(obj)
            }
            _ => {
                let index = self.source.read_short()? as usize;
                self.constant_pool.get(index).cloned().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Pool entry not found")
                })
            }
        }
    }

    fn read_pool_entry(&mut self, obj_type: u8) -> io::Result<Arc<dyn Any + Send + Sync>> {
        match obj_type {
            POOL_STRING => {
                let s = self.source.read_string()?;
                Ok(Arc::new(s))
            }
            POOL_ENUM => {
                let klass = self.read_pool_object()?;
                let ordinal = self.source.read_int()?;
                let enum_klass = if let Some(ek) = klass.downcast_ref::<EnumKlass>() {
                    ek.clone()
                } else {
                    EnumKlass {
                        name: String::new(),
                        values: Vec::new(),
                    }
                };
                Ok(Arc::new(EnumValue {
                    enum_klass,
                    ordinal,
                }))
            }
            POOL_CLASS => {
                let type_name = self.source.read_string()?;
                let kind = self.source.read_byte()?;
                if kind == ENUM_KLASS {
                    let count = self.source.read_int()? as usize;
                    let mut values = Vec::with_capacity(count);
                    for _ in 0..count {
                        let name_obj = self.read_pool_object()?;
                        if let Some(s) = name_obj.downcast_ref::<String>() {
                            values.push(s.clone());
                        }
                    }
                    Ok(Arc::new(EnumKlass {
                        name: type_name,
                        values,
                    }))
                } else {
                    Ok(Arc::new(Klass { name: type_name }))
                }
            }
            POOL_METHOD => {
                let holder_obj = self.read_pool_object()?;
                let holder = if let Some(k) = holder_obj.downcast_ref::<Klass>() {
                    k.clone()
                } else {
                    Klass {
                        name: String::new(),
                    }
                };
                let name = self.read_pool_object()?;
                let name = if let Some(s) = name.downcast_ref::<String>() {
                    s.clone()
                } else {
                    String::new()
                };
                let sig_obj = self.read_pool_object()?;
                let signature = if let Some(s) = sig_obj.downcast_ref::<Signature>() {
                    s.clone()
                } else {
                    Signature::new(Vec::new(), String::new())
                };
                let access_flags = self.source.read_int()?;
                let code = self.source.read_bytes()?;
                Ok(Arc::new(Method {
                    holder,
                    name,
                    signature,
                    access_flags,
                    code,
                }))
            }
            POOL_FIELD => {
                let holder_obj = self.read_pool_object()?;
                let holder = if let Some(k) = holder_obj.downcast_ref::<Klass>() {
                    k.clone()
                } else {
                    Klass {
                        name: String::new(),
                    }
                };
                let name = self.read_pool_object()?;
                let name = if let Some(s) = name.downcast_ref::<String>() {
                    s.clone()
                } else {
                    String::new()
                };
                let field_type = self.read_pool_object()?;
                let field_type = if let Some(s) = field_type.downcast_ref::<String>() {
                    s.clone()
                } else {
                    String::new()
                };
                let access_flags = self.source.read_int()?;
                Ok(Arc::new(Field {
                    holder,
                    name,
                    field_type,
                    access_flags,
                }))
            }
            POOL_SIGNATURE => {
                let arg_count = self.source.read_short()? as usize;
                let mut arg_types = Vec::with_capacity(arg_count);
                for _ in 0..arg_count {
                    let obj = self.read_pool_object()?;
                    if let Some(s) = obj.downcast_ref::<String>() {
                        arg_types.push(s.clone());
                    }
                }
                let ret_obj = self.read_pool_object()?;
                let return_type = if let Some(s) = ret_obj.downcast_ref::<String>() {
                    s.clone()
                } else {
                    String::new()
                };
                Ok(Arc::new(Signature {
                    arg_types,
                    return_type,
                }))
            }
            POOL_NODE_CLASS => {
                let klass_obj = self.read_pool_object()?;
                let class_name = if let Some(k) = klass_obj.downcast_ref::<Klass>() {
                    k.name.clone()
                } else if let Some(s) = klass_obj.downcast_ref::<String>() {
                    s.clone()
                } else {
                    String::new()
                };
                let name_template = self.source.read_string()?;
                let inputs = self.read_edges_info(true)?;
                let sux = self.read_edges_info(false)?;
                let sux_ports = sux.into_iter().map(|tp| tp.port).collect();
                Ok(Arc::new(NodeClass::new(
                    class_name,
                    name_template,
                    inputs,
                    sux_ports,
                )))
            }
            POOL_NODE_SOURCE_POSITION => {
                let _method = self.read_pool_object()?;
                let bci = self.source.read_int()?;
                // Read location data for version >= 6
                loop {
                    let uri_obj = self.read_pool_object()?;
                    if uri_obj.downcast_ref::<String>().is_none() {
                        break;
                    }
                    let _language = self.source.read_string()?;
                    let _line = self.source.read_int()?;
                    let _start = self.source.read_int()?;
                    let _end = self.source.read_int()?;
                }
                let _caller = self.read_pool_object()?;
                Ok(Arc::new((bci,)))
            }
            POOL_NODE => {
                let node_id = self.source.read_int()?;
                let _node_class = self.read_pool_object()?;
                Ok(Arc::new((node_id,)))
            }
            POOL_NULL => Ok(Arc::new(())),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown pool entry type: {}", obj_type),
            )),
        }
    }

    fn read_edges_info(&mut self, dump_inputs: bool) -> io::Result<Vec<TypedPort>> {
        let size = self.source.read_short()? as usize;
        let mut result = Vec::with_capacity(size);
        for _ in 0..size {
            let direct = self.source.read_byte()? == 0;
            let name_obj = self.read_pool_object()?;
            let name = if let Some(s) = name_obj.downcast_ref::<String>() {
                s.clone()
            } else {
                String::new()
            };
            let type_value = if dump_inputs {
                Some(self.read_pool_object()?)
            } else {
                None
            };
            result.push(TypedPort::new(!direct, name, type_value));
        }
        Ok(result)
    }

    /// Reads a property object from the stream.
    pub fn read_property_object(&mut self) -> io::Result<Arc<dyn Any + Send + Sync>> {
        let prop_type = self.source.read_byte()?;
        match prop_type {
            PROPERTY_POOL => self.read_pool_object(),
            PROPERTY_INT => Ok(Arc::new(self.source.read_int()?)),
            PROPERTY_LONG => Ok(Arc::new(self.source.read_long()?)),
            PROPERTY_DOUBLE => Ok(Arc::new(self.source.read_double()?)),
            PROPERTY_FLOAT => Ok(Arc::new(self.source.read_float()?)),
            PROPERTY_TRUE => Ok(Arc::new(true)),
            PROPERTY_FALSE => Ok(Arc::new(false)),
            PROPERTY_ARRAY => {
                let elem_type = self.source.read_byte()?;
                match elem_type {
                    PROPERTY_DOUBLE => {
                        let arr = self.source.read_doubles()?;
                        Ok(Arc::new(arr))
                    }
                    PROPERTY_INT => {
                        let arr = self.source.read_ints()?;
                        Ok(Arc::new(arr))
                    }
                    _ => {
                        let len = self.source.read_int()? as usize;
                        let mut arr = Vec::with_capacity(len);
                        for _ in 0..len {
                            arr.push(self.read_pool_object()?);
                        }
                        Ok(Arc::new(arr))
                    }
                }
            }
            PROPERTY_SUBGRAPH => Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Subgraph reading not yet implemented",
            )),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown property type: {}", prop_type),
            )),
        }
    }
}
