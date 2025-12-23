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
use bpx::table::column::Type;
use safer_ffi::prelude::*;
use crate::table::core::Table;
use crate::tree::model::Value;

#[derive_ReprC]
#[repr(opaque)]
pub struct Row {
    pub(super) inner: bpx::table::row::Row,
    value: Box<[Value]>
}

impl Row {
    fn new(inner: bpx::table::row::Row, table: &Table) -> Self {
        let container = unsafe { &*table.container };
        let mut val = Self {
            inner,
            value: vec![Value::Null; table.inner.columns(&container.underlying).len()].into_boxed_slice()
        };
        val.init(table);
        val
    }

    fn init(&mut self, table: &Table) {
        let container = unsafe { &*table.container };
        let columns = table.inner.columns(&container.underlying);
        for (index, cell) in self.value.iter_mut().enumerate() {
            match columns[index].ty {
                Type::Null => *cell = Value::Null,
                Type::Boolean => *cell = Value::Boolean(false),
                Type::Uint8 => *cell = Value::UInt8(0),
                Type::Uint16 => *cell = Value::UInt16(0),
                Type::Uint32 => *cell = Value::UInt32(0),
                Type::Uint64 => *cell = Value::UInt64(0),
                Type::Int8 => *cell = Value::Int8(0),
                Type::Int16 => *cell = Value::Int16(0),
                Type::Int32 => *cell = Value::Int32(0),
                Type::Int64 => *cell = Value::Int64(0),
                Type::Float => *cell = Value::Float(0.0),
                Type::Double => *cell = Value::Double(0.0),
                Type::Varchar => *cell = Value::String(CString::new("").unwrap().into()),
            }
        }
    }

    pub(super) fn sync_read(&mut self, table: &Table) {
        let container = unsafe { &*table.container };
        let columns = table.inner.columns(&container.underlying);
        for (index, cell) in self.value.iter_mut().enumerate() {
            let pos = table.inner.get_column_pos_at(index).unwrap();
            let r = self.inner.cell(pos);
            match columns[index].ty {
                Type::Null => (),
                Type::Boolean => *cell = Value::Boolean(r.get().unwrap()),
                Type::Uint8 => *cell = Value::UInt8(r.get().unwrap()),
                Type::Uint16 => *cell = Value::UInt16(r.get().unwrap()),
                Type::Uint32 => *cell = Value::UInt32(r.get().unwrap()),
                Type::Uint64 => *cell = Value::UInt64(r.get().unwrap()),
                Type::Int8 => *cell = Value::Int8(r.get().unwrap()),
                Type::Int16 => *cell = Value::Int16(r.get().unwrap()),
                Type::Int32 => *cell = Value::Int32(r.get().unwrap()),
                Type::Int64 => *cell = Value::Int64(r.get().unwrap()),
                Type::Float => *cell = Value::Float(r.get::<f64>().unwrap() as _),
                Type::Double => *cell = Value::Double(r.get().unwrap()),
                Type::Varchar => *cell = Value::String(CString::new(r.get::<&str>().unwrap()).unwrap().into()),
            }
        }
    }

    pub(super) fn sync_write(&mut self, table: &Table) {
        let container = unsafe { &*table.container };
        let columns = table.inner.columns(&container.underlying);
        for (index, cell) in self.value.iter().enumerate() {
            let pos = table.inner.get_column_pos_at(index).unwrap();
            let mut r = self.inner.cell_mut(pos);
            match columns[index].ty {
                Type::Null => (),
                Type::Boolean => r.set(cell.as_bool()).unwrap(),
                Type::Uint8 => r.set(cell.as_u64()).unwrap(),
                Type::Uint16 => r.set(cell.as_u64()).unwrap(),
                Type::Uint32 => r.set(cell.as_u64()).unwrap(),
                Type::Uint64 => r.set(cell.as_u64()).unwrap(),
                Type::Int8 => r.set(cell.as_i64()).unwrap(),
                Type::Int16 => r.set(cell.as_i64()).unwrap(),
                Type::Int32 => r.set(cell.as_i64()).unwrap(),
                Type::Int64 => r.set(cell.as_i64()).unwrap(),
                Type::Float => r.set(cell.as_f64()).unwrap(),
                Type::Double => r.set(cell.as_f64()).unwrap(),
                Type::Varchar => r.set(cell.as_str()).unwrap(),
            }
        }
    }
}

#[ffi_export]
pub fn bpx_table_row_create(table: &Table) -> repr_c::Box<Row> {
    Box::new(Row::new(table.inner.alloc_row(), table)).into()
}

#[ffi_export]
pub fn bpx_table_row_set_free(row: &mut Row, flag: bool) {
    row.inner.set_free(flag);
}

#[ffi_export]
pub fn bpx_table_row_is_free(row: &Row) -> bool {
    row.inner.is_free()
}

#[ffi_export]
pub fn bpx_table_row_get_value_const(row: &Row, index: usize) -> &Value {
    &row.value[index]
}

#[ffi_export]
pub fn bpx_table_row_get_value(row: &mut Row, index: usize) -> &mut Value {
    &mut row.value[index]
}

#[ffi_export]
pub fn bpx_table_row_destroy(row: repr_c::Box<Row>) {
    drop(row);
}

