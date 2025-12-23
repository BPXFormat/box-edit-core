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
use bpx::core::Handle;
use bpx::table::column::Type;
use bpx::table::core::RawTable;
use safer_ffi::prelude::*;
use crate::common::Container;
use crate::error::{set_last_error, unwrap_result};
use crate::table::row::Row;
use crate::tree::model::ValueType;

#[derive_ReprC]
#[repr(opaque)]
pub struct Table {
    pub(super) inner: RawTable,
    name: char_p::Box,
    pub(super) container: *const Container
}

#[ffi_export]
pub fn bpx_table_create(container: &mut Container, strings: u32, name: char_p::Ref<'_>) -> Option<repr_c::Box<Table>> {
    let strings = unsafe { Handle::from_raw(strings) };
    let inner = unwrap_result(RawTable::create(&mut container.underlying, name.to_str(), strings))?;
    container.refresh();
    Some(Box::new(Table { inner, name: name.to_owned(), container }).into())
}

#[ffi_export]
pub fn bpx_table_open(container: &Container, handle: u32, strings: u32) -> Option<repr_c::Box<Table>> {
    let handle = unsafe { Handle::from_raw(handle) };
    let strings = unsafe { Handle::from_raw(strings) };
    let inner = unwrap_result(RawTable::open(&container.underlying, handle, strings))?;
    let name = unwrap_result(inner.load_name(&container.underlying))?;
    let name = CString::new(name).unwrap();
    Some(Box::new(Table { inner, name: name.into(), container }).into())
}

#[ffi_export]
pub fn bpx_table_get_name(table: &Table) -> char_p::Ref<'_> {
    table.name.as_ref()
}

#[ffi_export]
pub fn bpx_table_save(table: &mut Table) -> bool {
    let container = unsafe { &*table.container };
    unwrap_result(table.inner.save(&container.underlying))
        .map(|()| true).unwrap_or(false)
}

#[ffi_export]
pub fn bpx_table_column_create(table: &mut Table, name: char_p::Ref<'_>, ty: ValueType, len: u16) -> isize {
    let container = unsafe { &*table.container };
    let ty = match ty {
        ValueType::Null => Type::Null,
        ValueType::Int8 => Type::Int8,
        ValueType::UInt8 => Type::Uint8,
        ValueType::Int16 => Type::Int16,
        ValueType::UInt16 => Type::Uint16,
        ValueType::Int32 => Type::Int32,
        ValueType::UInt32 => Type::Uint32,
        ValueType::Int64 => Type::Int64,
        ValueType::UInt64 => Type::Uint64,
        ValueType::Float => Type::Float,
        ValueType::Double => Type::Double,
        ValueType::Boolean => Type::Boolean,
        ValueType::String => Type::Varchar
    };
    match unwrap_result(table.inner.columns_mut(&container.underlying).create(name.to_str(), ty, len)) {
        Some(index) => index as isize,
        None => -1
    }
}

#[ffi_export]
pub fn bpx_table_column_remove_at(table: &mut Table, index: isize) {
    let container = unsafe { &*table.container };
    table.inner.columns_mut(&container.underlying).remove_at(index as usize);
}

#[ffi_export]
pub fn bpx_table_get_row_size(table: &Table) -> usize {
    table.inner.get_row_size()
}

#[ffi_export]
pub fn bpx_table_get_actual_row_size(table: &Table) -> usize {
    table.inner.get_actual_row_size()
}

#[ffi_export]
pub fn bpx_table_handle(table: &Table) -> u32 {
    table.inner.handle().into_raw()
}

#[ffi_export]
pub fn bpx_table_read(table: &Table, row: &mut Row, index: isize) -> bool {
    let container = unsafe { &*table.container };
    let data = unwrap_result(container.underlying.sections().load(table.inner.handle()));
    match data {
        None => false,
        Some(mut value) => {
            let res = unwrap_result(bpx::table::row::read(&mut *value, &mut row.inner, index as _)).map(|()| true).unwrap_or(false);
            row.sync_read(table);
            res
        }
    }
}

#[ffi_export]
pub fn bpx_table_write(table: &Table, row: &mut Row, index: isize) -> bool {
    row.sync_write(table);
    let container = unsafe { &*table.container };
    let data = unwrap_result(container.underlying.sections().load(table.inner.handle()));
    match data {
        None => false,
        Some(mut value) => {
            unwrap_result(bpx::table::row::write(&mut *value, &row.inner, index as _)).map(|()| true).unwrap_or(false)
        }
    }
}

#[ffi_export]
pub fn bpx_table_append(table: &Table, row: &mut Row) -> isize {
    row.sync_write(table);
    let container = unsafe { &*table.container };
    let data = unwrap_result(container.underlying.sections().load(table.inner.handle()));
    match data {
        None => -1,
        Some(mut value) => {
            unwrap_result(bpx::table::row::append(&mut *value, &row.inner)).map(|v| v as _).unwrap_or(-1)
        }
    }
}

#[ffi_export]
pub fn bpx_table_get_row_count(table: &Table, row: &Row) -> isize {
    let container = unsafe { &*table.container };
    let data = unwrap_result(container.underlying.sections().load(table.inner.handle()));
    match data {
        None => -1,
        Some(value) => bpx::table::row::count(&*value, &row.inner) as _
    }
}

#[ffi_export]
pub fn bpx_table_get_column_index(table: &Table, name: char_p::Ref<'_>) -> isize {
    let container = unsafe { &*table.container };
    let columns = table.inner.columns(&container.underlying);
    let column = unwrap_result(columns.find(name.to_str()));
    match column {
        Some(v) => match v {
            Some(v) => {
                for (index, column) in columns.iter().enumerate() {
                    if column.name == v.name {
                        return index as _;
                    }
                }
                unreachable!();
            }
            None => {
                set_last_error(bpx::table::error::Error::ColumnNotFound(name.to_str().into()));
                -1
            }
        }
        None => -1,
    }
}

#[ffi_export]
pub fn bpx_table_get_columns(table: &Table) -> usize {
    let container = unsafe { &*table.container };
    table.inner.columns(&container.underlying).len()
}

#[ffi_export]
pub fn bpx_table_destroy(table: repr_c::Box<Table>) {
    drop(table);
}
