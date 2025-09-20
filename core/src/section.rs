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

use std::io::{Read, Seek, SeekFrom, Write};
use bpx::core::{AutoSectionData, Handle, SectionData};
use bpx::core::header::{FLAG_CHECK_CRC32, FLAG_CHECK_WEAK, FLAG_COMPRESS_XZ, FLAG_COMPRESS_ZLIB};
use bpx::core::options::{Checksum, CompressionMethod};
use bpx::traits::{Shift, ShiftTo};
use safer_ffi::prelude::*;
use crate::common::{Container, CSeekFrom, SectionInfo};
use crate::error::{IntoBPXError, RustError, unwrap_result};

#[derive_ReprC]
#[repr(C)]
pub struct SectionOptions {
    ty: u8,
    flags: u8,
    compression_threshold: isize,
}

impl SectionOptions {
    fn to_options(&self) -> bpx::core::options::SectionOptions {
        let mut opts = bpx::core::options::SectionOptions::default();
        opts.ty(self.ty);
        if self.flags & FLAG_CHECK_WEAK != 0 {
            opts.checksum(Checksum::Weak);
        }
        if self.flags & FLAG_CHECK_CRC32 != 0 {
            opts.checksum(Checksum::Crc32);
        }
        if self.flags & FLAG_COMPRESS_ZLIB != 0 {
            opts.compression(CompressionMethod::Zlib);
        }
        if self.flags & FLAG_COMPRESS_XZ != 0 {
            opts.compression(CompressionMethod::Xz);
        }
        if self.compression_threshold >= 0 && (self.flags & FLAG_COMPRESS_ZLIB != 0 || self.flags & FLAG_COMPRESS_XZ != 0) {
            opts.threshold(self.compression_threshold as u32);
        }
        opts
    }
}

#[ffi_export]
pub unsafe fn bpx_section_options_default(options: *mut SectionOptions) {
    *options = SectionOptions {
        ty: 0,
        flags: FLAG_CHECK_WEAK | FLAG_COMPRESS_ZLIB,
        compression_threshold: -1,
    }
}

#[ffi_export]
pub fn bpx_section_create(container: &mut Container, options: &SectionOptions) -> u32 {
    let handle = container.underlying.sections_mut().create(options.to_options());
    container.sections.push(SectionInfo::from((handle, &container.underlying.sections()[handle])));
    container.main_header.section_num += 1;
    handle.into_raw()
}

#[ffi_export]
pub fn bpx_section_remove(container: &mut Container, handle: u32) {
    let handle = unsafe { Handle::from_raw(handle) };
    container.underlying.sections_mut().remove(handle);
    container.refresh();
}

fn try_with_section<E: IntoBPXError + Into<RustError>, T, F: FnOnce(&mut AutoSectionData) -> Result<T, E>>(container: &Container, handle: u32, closure: F) -> Option<T> {
    let handle = unsafe { Handle::from_raw(handle) };
    let mut v = unwrap_result(container.underlying.sections().load(handle))?;
    unwrap_result(closure(&mut v))
}

fn with_section<T, F: FnOnce(&mut AutoSectionData) -> T>(container: &Container, handle: u32, closure: F) -> Option<T> {
    let handle = unsafe { Handle::from_raw(handle) };
    let mut v = unwrap_result(container.underlying.sections().load(handle))?;
    Some(closure(&mut v))
}

#[ffi_export]
pub fn bpx_section_size(container: &Container, handle: u32) -> isize {
    with_section(container, handle, |v| {
        v.size()
    }).map(|v| { v as _ }).unwrap_or(-1)
}

#[ffi_export]
pub fn bpx_section_seek(container: &Container, handle: u32, from: CSeekFrom, pos: isize) -> isize {
    try_with_section(container, handle, |v| {
        match from {
            CSeekFrom::Start => v.seek(SeekFrom::Start(pos as _)),
            CSeekFrom::Current => v.seek(SeekFrom::Current(pos as _)),
            CSeekFrom::End => v.seek(SeekFrom::End(pos as _))
        }
    }).map(|v| { v as _ }).unwrap_or(-1)
}

#[ffi_export]
pub fn bpx_section_read(container: &Container, handle: u32, buffer: c_slice::Mut<'_, u8>) -> isize {
    try_with_section(container, handle, |v| v.read(buffer.as_slice()))
        .map(|v| v as _)
        .unwrap_or(-1)
}

#[ffi_export]
pub fn bpx_section_write(container: &Container, handle: u32, buffer: c_slice::Ref<'_, u8>) -> isize {
    try_with_section(container, handle, |v| v.write(buffer.as_slice()))
        .map(|v| v as _)
        .unwrap_or(-1)
}

#[ffi_export]
pub fn bpx_section_shift_right(container: &Container, handle: u32, length: usize) -> bool {
    try_with_section(container, handle, |v| v.shift(ShiftTo::Right(length as _)))
        .map(|_| true)
        .unwrap_or(false)
}

#[ffi_export]
pub fn bpx_section_shift_left(container: &Container, handle: u32, length: usize) -> bool {
    try_with_section(container, handle, |v| v.shift(ShiftTo::Left(length as _)))
        .map(|_| true)
        .unwrap_or(false)
}
