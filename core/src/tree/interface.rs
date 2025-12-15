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

use crate::tree::model::{Node, Value, ValueType};
use safer_ffi::prelude::*;

#[ffi_export]
pub fn bpx_node_free(node: repr_c::Box<Node>) {
    drop(node);
}

#[ffi_export]
pub fn bpx_node_get_name(node: &Node) -> char_p::Ref<'_> {
    node.name.as_ref()
}

#[ffi_export]
pub fn bpx_node_get_value(node: &Node) -> &Value {
    &node.value
}

#[ffi_export]
pub fn bpx_node_get_details(node: &Node) -> Option<&Node> {
    node.details.as_deref()
}

#[ffi_export]
pub fn bpx_node_get_children(node: &Node) -> c_slice::Ref<'_, Node> {
    node.children.as_slice().into()
}

#[ffi_export]
pub fn bpx_value_get_type(value: &Value) -> ValueType {
    value.get_type()
}

#[ffi_export]
pub fn bpx_value_is_null(value: &Value) -> bool {
    value.get_type() == ValueType::Null
}

#[ffi_export]
pub fn bpx_value_get_int8(value: &Value) -> i8 {
    match value {
        Value::Int8(v) => *v,
        _ => 0
    }
}

#[ffi_export]
pub fn bpx_value_get_uint8(value: &Value) -> u8 {
    match value {
        Value::UInt8(v) => *v,
        _ => 0
    }
}

#[ffi_export]
pub fn bpx_value_get_int16(value: &Value) -> i16 {
    match value {
        Value::Int16(v) => *v,
        _ => 0
    }
}

#[ffi_export]
pub fn bpx_value_get_uint16(value: &Value) -> u16 {
    match value {
        Value::UInt16(v) => *v,
        _ => 0
    }
}

#[ffi_export]
pub fn bpx_value_get_int32(value: &Value) -> i32 {
    match value {
        Value::Int32(v) => *v,
        _ => 0
    }
}

#[ffi_export]
pub fn bpx_value_get_uint32(value: &Value) -> u32 {
    match value {
        Value::UInt32(v) => *v,
        _ => 0
    }
}

#[ffi_export]
pub fn bpx_value_get_int64(value: &Value) -> i64 {
    match value {
        Value::Int64(v) => *v,
        _ => 0
    }
}

#[ffi_export]
pub fn bpx_value_get_uint64(value: &Value) -> u64 {
    match value {
        Value::UInt64(v) => *v,
        _ => 0
    }
}

#[ffi_export]
pub fn bpx_value_get_float(value: &Value) -> f32 {
    match value {
        Value::Float(v) => *v,
        _ => 0.0
    }
}

#[ffi_export]
pub fn bpx_value_get_double(value: &Value) -> f64 {
    match value {
        Value::Double(v) => *v,
        _ => 0.0
    }
}

#[ffi_export]
pub fn bpx_value_get_boolean(value: &Value) -> bool {
    match value {
        Value::Boolean(v) => *v,
        _ => false
    }
}

#[ffi_export]
pub fn bpx_value_get_string(value: &Value) -> Option<char_p::Ref<'_>> {
    match value {
        Value::String(v) => Some(v.as_ref()),
        _ => None
    }
}
