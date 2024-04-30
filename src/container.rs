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

use bpx::core::DEFAULT_MEMORY_THRESHOLD;
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
    pub memory_threshold: u32
}

#[derive_ReprC]
#[repr(C)]
pub struct CreateOptions {
    pub flags: u8,
    pub memory_threshold: u32,
    pub main_header: MainHeader
}

#[ffi_export]
pub unsafe fn bpx_create_options_default(options: *mut CreateOptions) {
    *options = CreateOptions {
        flags: 0,
        memory_threshold: DEFAULT_MEMORY_THRESHOLD,
        main_header: MainHeader::from(bpx::core::header::MainHeader::new())
    }
}

#[ffi_export]
pub unsafe fn bpx_open_options_default(options: *mut OpenOptions) {
    *options = OpenOptions {
        flags: 0,
        memory_threshold: DEFAULT_MEMORY_THRESHOLD
    }
}

#[ffi_export]
pub fn bpx_container_create(stream: repr_c::Box<Stream>, options: &CreateOptions) -> repr_c::Box<Container> {
    let opts = bpx::core::options::CreateOptions::new(*stream.into())
        .ty(options.main_header.ty)
        .version(options.main_header.version)
        .type_ext(options.main_header.type_ext)
        .memory_threshold(options.memory_threshold)
        .revert_on_save_failure((options.flags & FLAG_REVERT_ON_SAVE_FAIL) != 0);
    let container = bpx::core::Container::create(opts);
    Box::new(Container::from(container)).into()
}

#[ffi_export]
pub fn bpx_container_open(stream: repr_c::Box<Stream>, options: &OpenOptions) -> Option<repr_c::Box<Container>> {
    let opts = bpx::core::options::OpenOptions::new(*stream.into())
        .memory_threshold(options.memory_threshold)
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
    unwrap_result(container.underlying.save()).is_some()
}

#[ffi_export]
pub fn bpx_container_close(container: repr_c::Box<Container>) {
    drop(container);
}

/*export! {
    fn container_create(data_stream: Id<ProtocolObject<dyn DataStream>>) -> *mut c_void {
        let ptr = DataStreamPtr::from(data_stream);
        let container = Container::create(ptr);
        let bx = Box::new(container);
        return Box::into_raw(bx) as _
    }

    fn container_open(data_stream: Id<ProtocolObject<dyn DataStream>>, flags: u8, out: *mut *mut c_void, error: *mut *mut NSError) -> BOOL {
        let ptr = DataStreamPtr::from(data_stream);
        let mut options = bpx::core::options::OpenOptions::new(ptr);
        if flags & FLAG_IGNORE_CHECKSUM != 0 {
            options = options.skip_checksum(true);
        }
        if flags & FLAG_IGNORE_SIGNATURE != 0 {
            options = options.skip_signature(true);
        }
        let container = Container::open(options).map_err(|e| e.into_ns_error());
        match container {
            Ok(v) => {
                let bx = Box::new(v);
                *out = Box::into_raw(bx) as _;
                return YES;
            },
            Err(e) => {
                *error = Id::autorelease_return(e);
                return NO;
            }
        }
    }

    fn container_close(container: *mut c_void) {
        unsafe {
            let bx: Box<ContainerPtr> = Box::from_raw(container as _);
            drop(bx);
        }
    }
}*/
