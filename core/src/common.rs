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

use bpx::core::{AutoSectionData, Handle};
use crate::stream::Stream;
use safer_ffi::prelude::*;
use crate::error::{unwrap_result, IntoBPXError, RustError};

#[derive_ReprC]
#[repr(i32)]
pub enum CSeekFrom {
    Start = 0,
    Current,
    End
}

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

impl Container {
    pub fn refresh(&mut self) {
        let sections = self.underlying.sections()
            .iter()
            .map(|v| SectionInfo::from((v, &self.underlying.sections()[v])))
            .collect();
        let main_header = MainHeader::from(self.underlying.main_header());
        self.sections = sections;
        self.main_header = main_header;
    }
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

pub fn try_with_section<E: IntoBPXError + Into<RustError>, T, F: FnOnce(&mut AutoSectionData) -> Result<T, E>>(container: &Container, handle: u32, closure: F) -> Option<T> {
    let handle = unsafe { Handle::from_raw(handle) };
    let mut v = unwrap_result(container.underlying.sections().load(handle))?;
    unwrap_result(closure(&mut v))
}

pub fn with_section<T, F: FnOnce(&mut AutoSectionData) -> T>(container: &Container, handle: u32, closure: F) -> Option<T> {
    let handle = unsafe { Handle::from_raw(handle) };
    let mut v = unwrap_result(container.underlying.sections().load(handle))?;
    Some(closure(&mut v))
}
