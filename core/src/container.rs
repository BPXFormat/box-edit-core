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

use bpx::core::{DEFAULT_MEMORY_THRESHOLD, Handle, DEFAULT_COMPRESSION_THRESHOLD};
use bpx::core::header::Struct;
use safer_ffi::prelude::*;
use crate::common::{Container, MainHeader, SectionInfo};
use crate::error::unwrap_result;
use crate::stream::Stream;

pub const FLAG_IGNORE_CHECKSUM: u8 = 0x1;
pub const FLAG_IGNORE_SIGNATURE: u8 = 0x2;
pub const FLAG_IGNORE_VERSION: u8 = 0x4;
pub const FLAG_REVERT_ON_SAVE_FAIL: u8 = 0x8;

#[derive_ReprC]
#[repr(C)]
pub struct OpenOptions {
    pub flags: u8,
    pub memory_threshold: u32,
    pub compression_threshold: u32
}

#[derive_ReprC]
#[repr(C)]
pub struct CreateOptions {
    pub flags: u8,
    pub memory_threshold: u32,
    pub compression_threshold: u32,
    pub main_header: MainHeader
}

#[ffi_export]
pub unsafe fn bpx_create_options_default(options: *mut CreateOptions) {
    *options = CreateOptions {
        flags: 0,
        memory_threshold: DEFAULT_MEMORY_THRESHOLD,
        compression_threshold: DEFAULT_COMPRESSION_THRESHOLD,
        main_header: MainHeader::from(bpx::core::header::MainHeader::new())
    }
}

#[ffi_export]
pub unsafe fn bpx_open_options_default(options: *mut OpenOptions) {
    *options = OpenOptions {
        flags: 0,
        memory_threshold: DEFAULT_MEMORY_THRESHOLD,
        compression_threshold: DEFAULT_COMPRESSION_THRESHOLD
    }
}

#[ffi_export]
pub fn bpx_container_create(stream: repr_c::Box<Stream>, options: &CreateOptions) -> repr_c::Box<Container> {
    let opts = bpx::core::options::CreateOptions::new(*stream.into())
        .ty(options.main_header.ty)
        .version(options.main_header.version)
        .type_ext(options.main_header.type_ext)
        .memory_threshold(options.memory_threshold)
        .compression_threshold(options.compression_threshold)
        .revert_on_save_failure((options.flags & FLAG_REVERT_ON_SAVE_FAIL) != 0);
    let container = bpx::core::Container::create(opts);
    Box::new(Container::from(container)).into()
}

#[ffi_export]
pub fn bpx_container_open(stream: repr_c::Box<Stream>, options: &OpenOptions) -> Option<repr_c::Box<Container>> {
    let opts = bpx::core::options::OpenOptions::new(*stream.into())
        .memory_threshold(options.memory_threshold)
        .compression_threshold(options.compression_threshold)
        .revert_on_save_failure((options.flags & FLAG_REVERT_ON_SAVE_FAIL) != 0)
        .skip_checksum((options.flags & FLAG_IGNORE_CHECKSUM) != 0)
        .skip_versions((options.flags & FLAG_IGNORE_VERSION) != 0)
        .skip_signature((options.flags & FLAG_IGNORE_SIGNATURE) != 0);
    unwrap_result(bpx::core::Container::open(opts)).map(|v| Box::new(Container::from(v)).into())
}

#[ffi_export]
pub fn bpx_container_get_main_header(container: &Container) -> &'_ MainHeader {
    &container.main_header
}

#[ffi_export]
pub fn bpx_container_get_sections(container: &Container) -> c_slice::Ref<'_, SectionInfo> {
    container.sections.as_slice().into()
}

#[ffi_export]
pub fn bpx_container_save(container: &mut Container) -> bool {
    match unwrap_result(container.underlying.save()) {
        Some(_) => {
            container.main_header = MainHeader::from(container.underlying.main_header());
            for v in &mut container.sections {
                let handle = unsafe { Handle::from_raw(v.handle) };
                *v = SectionInfo::from((handle, &container.underlying.sections()[handle]));
            }
            true
        },
        None => false
    }
}

#[ffi_export]
pub fn bpx_container_close(container: repr_c::Box<Container>) {
    drop(container);
}
