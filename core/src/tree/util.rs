// Copyright 2024 Yuri6037
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy
// of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
// THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS
// IN THE SOFTWARE.

use std::ffi::CString;
use bp3d_sdk_util::simple_error;
use crate::tree::model::{Node, Value};
use std::fmt::{Display, Formatter};

simple_error! {
    Error {
        UnsupportedValue => "unsupported value",
        InvalidString => "invalid string",
        TypeError(bpx::sd::error::TypeError) => "BPXSD type error {}"
    }
}

impl TryFrom<&bpx::sd::Value> for Value {
    type Error = Error;

    fn try_from(value: &bpx::sd::Value) -> Result<Self, Self::Error> {
        match value {
            bpx::sd::Value::Null => Ok(Value::Null),
            bpx::sd::Value::Bool(v) => Ok(Value::Boolean(*v)),
            bpx::sd::Value::Uint8(v) => Ok(Value::UInt8(*v)),
            bpx::sd::Value::Uint16(v) => Ok(Value::UInt16(*v)),
            bpx::sd::Value::Uint32(v) => Ok(Value::UInt32(*v)),
            bpx::sd::Value::Uint64(v) => Ok(Value::UInt64(*v)),
            bpx::sd::Value::Int8(v) => Ok(Value::Int8(*v)),
            bpx::sd::Value::Int16(v) => Ok(Value::Int16(*v)),
            bpx::sd::Value::Int32(v) => Ok(Value::Int32(*v)),
            bpx::sd::Value::Int64(v) => Ok(Value::Int64(*v)),
            bpx::sd::Value::Float(v) => Ok(Value::Float(*v)),
            bpx::sd::Value::Double(v) => Ok(Value::Double(*v)),
            bpx::sd::Value::String(v) => v.clone().try_into().map(Value::String).map_err(|_| Error::InvalidString),
            bpx::sd::Value::Array(_) => Err(Error::UnsupportedValue),
            bpx::sd::Value::Object(_) => Err(Error::UnsupportedValue)
        }
    }
}

impl TryFrom<&bpx::sd::Array> for Node {
    type Error = Error;

    fn try_from(value: &bpx::sd::Array) -> Result<Self, Self::Error> {
        let mut node = Node {
            name: CString::from(c"").into(),
            value: Value::Null,
            details: None,
            children: Vec::new()
        };
        for v in value {
            let n = match v {
                bpx::sd::Value::Array(v) => Node::try_from(v)?,
                bpx::sd::Value::Object(v) => Node::try_from(v)?,
                v => Node {
                    name: CString::from(c"").into(),
                    value: v.try_into()?,
                    details: None,
                    children: Vec::new()
                }
            };
            node.children.push(n);
        }
        Ok(node)
    }
}

impl TryFrom<&bpx::sd::Object> for Node {
    type Error = Error;

    fn try_from(value: &bpx::sd::Object) -> Result<Self, Self::Error> {
        let mut node = Node {
            name: CString::from(c"root").into(),
            value: Value::Null,
            details: None,
            children: Vec::new()
        };
        let debugger = bpx::sd::debug::Debugger::attach(value).map_err(Error::TypeError)?;
        for (name, hash, value) in &debugger {
            let child = match value {
                bpx::sd::Value::Array(v) => {
                    let mut node: Node = v.try_into()?;
                    node.name = name.map(String::from)
                        .unwrap_or_else(|| format!("{:X}", hash.into_inner())).try_into()
                        .map_err(|_| Error::InvalidString)?;
                    node
                },
                bpx::sd::Value::Object(v) => {
                    let mut node: Node = v.try_into()?;
                    node.name = name.map(String::from)
                        .unwrap_or_else(|| format!("{:X}", hash.into_inner())).try_into()
                        .map_err(|_| Error::InvalidString)?;
                    node
                },
                v => Node {
                    value: v.try_into()?,
                    name: name.map(String::from)
                        .unwrap_or_else(|| format!("{:X}", hash.into_inner())).try_into()
                        .map_err(|_| Error::InvalidString)?,
                    details: None,
                    children: Vec::new()
                }
            };
            node.children.push(child);
        }
        Ok(node)
    }
}
