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

use bpx::core::Handle;
use crate::stream::Stream;
use safer_ffi::prelude::*;

#[derive_ReprC]
#[repr(C)]
pub struct SectionHeader {
    pub pointer: u64,
    pub csize: u32,
    pub size: u32,
    pub chksum: u32,
    pub ty: u8,
    pub flags: u8,
}

impl From<&bpx::core::header::SectionHeader> for SectionHeader {
    fn from(value: &bpx::core::header::SectionHeader) -> Self {
        Self {
            pointer: value.pointer,
            csize: value.csize,
            size: value.size,
            chksum: value.chksum,
            ty: value.ty,
            flags: value.flags,
        }
    }
}

#[derive_ReprC]
#[repr(C)]
pub struct SectionInfo {
    pub header: SectionHeader,
    pub index: u32,
    pub handle: u32
}

impl From<(Handle, &bpx::core::SectionInfo)> for SectionInfo {
    fn from((handle, value): (Handle, &bpx::core::SectionInfo)) -> Self {
        Self {
            header: value.header().into(),
            index: value.index(),
            handle: handle.into_raw()
        }
    }
}

#[derive_ReprC]
#[repr(C)]
pub struct MainHeader {
    pub signature: [u8; 3],
    pub ty: u8,
    pub chksum: u32,
    pub file_size: u64,
    pub section_num: u32,
    pub version: u32,
    pub type_ext: [u8; 16],
}

impl From<&bpx::core::header::MainHeader> for MainHeader {
    fn from(value: &bpx::core::header::MainHeader) -> Self {
        Self {
            signature: value.signature,
            ty: value.ty,
            chksum: value.chksum,
            file_size: value.file_size,
            section_num: value.section_num,
            version: value.version,
            type_ext: value.type_ext.into_inner()
        }
    }
}

impl From<bpx::core::header::MainHeader> for MainHeader {
    fn from(value: bpx::core::header::MainHeader) -> Self {
        Self::from(&value)
    }
}

#[derive_ReprC]
#[repr(opaque)]
pub struct Container {
    pub underlying: bpx::core::Container<Stream>,
    pub sections: Vec<SectionInfo>,
    pub main_header: MainHeader
}

impl From<bpx::core::Container<Stream>> for Container {
    fn from(value: bpx::core::Container<Stream>) -> Self {
        let sections = value.sections()
            .iter()
            .map(|v| SectionInfo::from((v, &value.sections()[v])))
            .collect();
        let main_header = MainHeader::from(value.main_header());
        Self {
            sections,
            main_header,
            underlying: value
        }
    }
}
