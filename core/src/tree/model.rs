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

use safer_ffi::prelude::*;

#[derive_ReprC]
#[repr(opaque)]
pub enum Value {
    Null,
    Int8(i8),
    UInt8(u8),
    Int16(i16),
    UInt16(u16),
    Int32(i32),
    UInt32(u32),
    Int64(i64),
    UInt64(u64),
    Float(f32),
    Double(f64),
    Boolean(bool),
    String(char_p::Box)
}

#[derive_ReprC]
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ValueType {
    Null = 0,
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Float,
    Double,
    Boolean,
    String
}

impl Value {
    pub fn get_type(&self) -> ValueType {
        match self {
            Value::Null => ValueType::Null,
            Value::Int8(_) => ValueType::Int8,
            Value::UInt8(_) => ValueType::UInt8,
            Value::Int16(_) => ValueType::Int16,
            Value::UInt16(_) => ValueType::UInt16,
            Value::Int32(_) => ValueType::Int32,
            Value::UInt32(_) => ValueType::UInt32,
            Value::Int64(_) => ValueType::Int64,
            Value::UInt64(_) => ValueType::UInt64,
            Value::Float(_) => ValueType::Float,
            Value::Double(_) => ValueType::Double,
            Value::Boolean(_) => ValueType::Boolean,
            Value::String(_) => ValueType::String,
        }
    }
}

#[derive_ReprC]
#[repr(opaque)]
pub struct Node {
    pub name: char_p::Box,
    pub value: Value,
    pub details: Option<repr_c::Box<Node>>,
    pub children: Vec<Node>
}
