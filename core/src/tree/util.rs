// Copyright (c) 2025, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::ffi::CString;
use bp3d_util::simple_error;
use crate::tree::model::{Node, Value};
use std::fmt::{Display, Formatter};

simple_error! {
    pub Error {
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
