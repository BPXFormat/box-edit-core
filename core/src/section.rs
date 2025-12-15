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

use std::io::{Read, Seek, SeekFrom, Write};
use bpx::core::{AutoSectionData, Handle, SectionData};
use bpx::core::header::{FLAG_CHECK_CRC32, FLAG_CHECK_WEAK, FLAG_COMPRESS_XZ, FLAG_COMPRESS_ZLIB};
use bpx::core::options::{Checksum, CompressionMethod};
use bpx::util::traits::{Shift, ShiftTo};
use safer_ffi::prelude::*;
use crate::common::{Container, CSeekFrom, SectionInfo, with_section, try_with_section};
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
