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

use std::ffi::c_void;
use std::io::{Read, Seek, SeekFrom, Write};
use bpx::core::{AutoSectionData, Handle, SectionData};
use bpx::core::header::{FLAG_CHECK_CRC32, FLAG_CHECK_WEAK, FLAG_COMPRESS_XZ, FLAG_COMPRESS_ZLIB, SectionHeader};
use bpx::core::options::{Checksum, CompressionMethod};
use bpx::traits::{Shift, ShiftTo};
use icrate::Foundation::{NSArray, NSData, NSError, NSMutableArray};
use objc2::{extern_class, mutability, ClassType, runtime::NSObject, msg_send_id, Encode, Encoding};
use objc2::ffi::{BOOL, NO, YES};
use objc2::rc::Id;
use crate::ContainerPtr;
use crate::error::{IntoNSError, NoneError};
use crate::util::export;

#[repr(C)]
pub struct BPXSectionOptions {
    ty: u8,
    flags: u8,
    compression_threshold: u32,
}

impl BPXSectionOptions {
    fn to_options(&self) -> bpx::core::options::SectionOptions {
        let mut opts = bpx::core::options::SectionOptions::new();
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
        if self.flags & FLAG_COMPRESS_ZLIB != 0 || self.flags & FLAG_COMPRESS_XZ != 0 {
            opts.threshold(self.compression_threshold);
        }
        opts
    }
}

#[repr(C)]
pub struct BPXSectionHeader {
    pointer: u64,
    csize: u32,
    size: u32,
    checksum: u32,
    ty: u8,
    flags: u8
}

impl BPXSectionHeader {
    fn from_header(header: &SectionHeader) -> Self {
        Self {
            pointer: header.pointer,
            csize: header.csize,
            size: header.size,
            checksum: header.chksum,
            ty: header.ty,
            flags: header.flags,
        }
    }
}

unsafe impl Encode for BPXSectionHeader {
    const ENCODING: Encoding = Encoding::Struct("_BPXSectionHeader", &[u64::ENCODING, u32::ENCODING, u32::ENCODING, u32::ENCODING, u8::ENCODING, u8::ENCODING]);
}

extern_class! {
    #[repr(C)]
    pub struct BPXSection;

    unsafe impl ClassType for BPXSection {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
}

trait FFIError {
    fn error() -> Self;
}

impl FFIError for BOOL {
    fn error() -> Self {
        NO
    }
}

impl<T> FFIError for *mut T {
    fn error() -> Self {
        std::ptr::null_mut()
    }
}

unsafe fn with_section<E: IntoNSError, T: FFIError, F: FnOnce(&mut AutoSectionData) -> Result<T, E>>(container: *const c_void, handle: u32, error: *mut *mut NSError, closure: F) -> T {
    let bpx = &*(container as *const ContainerPtr);
    match bpx.sections().load(Handle::from_raw(handle)).map_err(|e| e.into_ns_error()) {
        Ok(mut v) => {
            match closure(&mut v).map_err(|e| e.into_ns_error()) {
                Err(e) => {
                    *error = Id::autorelease_return(e);
                    T::error()
                },
                Ok(v) => {
                    v
                }
            }
        },
        Err(e) => {
            *error = Id::autorelease_return(e);
            T::error()
        }
    }
}

export! {
    fn section_create(container: *mut c_void, options: *const BPXSectionOptions) -> *mut BPXSection {
        let bpx = &mut *(container as *mut ContainerPtr);
        let opts = (*options).to_options();
        let handle = bpx.sections_mut().create(opts);
        let header = BPXSectionHeader::from_header(bpx.sections()[handle].header());
        let index = bpx.sections()[handle].index();
        let section: Id<BPXSection> = msg_send_id![BPXSection::alloc(), initWithContainer: container handle: handle.into_raw() header: header index: index];
        Id::autorelease_return(section)
    }

    fn section_list(container: *const c_void) -> *mut NSArray {
        let bpx = &*(container as *const ContainerPtr);
        let mut array = NSMutableArray::new();
        for handle in bpx.sections() {
            let header = BPXSectionHeader::from_header(bpx.sections()[handle].header());
            let index = bpx.sections()[handle].index();
            let section: Id<BPXSection> = msg_send_id![BPXSection::alloc(), initWithContainer: container handle: handle.into_raw() header: header index: index];
            array.addObject(&*section);
        }
        Id::autorelease_return(Id::cast(array))
    }

    fn section_size(container: *const c_void, handle: u32, error: *mut *mut NSError, size: *mut usize) -> BOOL {
        with_section::<NoneError, _, _>(container, handle, error, |data| {
            *size = data.size();
            Ok(YES)
        })
    }

    fn section_seek(container: *const c_void, handle: u32, error: *mut *mut NSError, pos: u64) -> BOOL {
        with_section(container, handle, error, |data| data.seek(SeekFrom::Start(pos)).map(|_| YES))
    }

    fn section_read(container: *const c_void, handle: u32, error: *mut *mut NSError, length: usize) -> *mut NSData {
        with_section::<std::io::Error, _, _>(container, handle, error, |data| {
            let mut buffer = vec![0u8; length];
            data.read(buffer.as_mut_slice())?;
            Ok(Id::autorelease_return(NSData::from_vec(buffer)))
        })
    }

    fn section_shift(container: *const c_void, handle: u32, error: *mut *mut NSError, length: usize) -> BOOL {
        with_section(container, handle, error, |data| data.shift(ShiftTo::Right(length as _)).map(|_| YES))
    }

    fn section_write(container: *const c_void, handle: u32, error: *mut *mut NSError, nsdata: *const NSData) -> BOOL {
        with_section(container, handle, error, |data| data.write((*nsdata).bytes()).map(|_| YES))
    }
}
