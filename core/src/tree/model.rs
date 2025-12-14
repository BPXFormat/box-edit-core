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
