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

use crate::tree::model::{Node, Value, ValueType};
use safer_ffi::prelude::*;

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
