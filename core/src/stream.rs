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

use safer_ffi::prelude::*;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use crate::common::CSeekFrom;
use crate::error::unwrap_result;

#[derive_ReprC(dyn)]
pub trait FfiStream {
    fn read(&mut self, buffer: c_slice::Mut<'_, u8>) -> isize;
    fn write(&mut self, buffer: c_slice::Ref<'_, u8>) -> isize;
    fn flush(&mut self) -> bool;
    fn seek(&mut self, from: CSeekFrom, pos: isize) -> isize;
}

pub struct FfiStreamWrapper(VirtualPtr<dyn FfiStream>);

impl Read for FfiStreamWrapper {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let res = self.0.read(buf.into());
        if res >= 0 {
            Ok(res as _)
        } else {
            Err(std::io::Error::last_os_error())
        }
    }
}

impl Write for FfiStreamWrapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let res = self.0.write(buf.into());
        if res >= 0 {
            Ok(res as _)
        } else {
            Err(std::io::Error::last_os_error())
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if self.0.flush() {
            Ok(())
        } else {
            Err(std::io::Error::last_os_error())
        }
    }
}

impl Seek for FfiStreamWrapper {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let res = match pos {
            SeekFrom::Start(v) => self.0.seek(CSeekFrom::Start, v as _ ),
            SeekFrom::End(v) => self.0.seek(CSeekFrom::End, v as _),
            SeekFrom::Current(v) => self.0.seek(CSeekFrom::Current, v as _)
        };
        if res >= 0 {
            Ok(res as _)
        } else {
            Err(std::io::Error::last_os_error())
        }
    }
}

#[derive_ReprC]
#[repr(opaque)]
pub enum Stream {
    File(File),
    Ffi(FfiStreamWrapper)
}

impl Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Stream::File(v) => v.read(buf),
            Stream::Ffi(v) => v.read(buf)
        }
    }
}

impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Stream::File(v) => v.write(buf),
            Stream::Ffi(v) => v.write(buf)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Stream::File(v) => v.flush(),
            Stream::Ffi(v) => v.flush()
        }
    }
}

impl Seek for Stream {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match self {
            Stream::File(v) => v.seek(pos),
            Stream::Ffi(v) => v.seek(pos)
        }
    }
}

#[ffi_export]
pub fn bpx_stream_create(path: char_p::Ref<'_>) -> Option<repr_c::Box<Stream>> {
    let path = Path::new(OsStr::from_bytes(path.to_bytes()));
    unwrap_result(File::create(path).map(|v| Box::new(Stream::File(v)).into()))
}

#[ffi_export]
pub fn bpx_stream_open(path: char_p::Ref<'_>) -> Option<repr_c::Box<Stream>> {
    let path = Path::new(OsStr::from_bytes(path.to_bytes()));
    unwrap_result(File::open(path).map(|v| Box::new(Stream::File(v)).into()))
}

#[ffi_export]
pub fn bpx_stream_new(stream: VirtualPtr<dyn FfiStream>) -> repr_c::Box<Stream> {
    Box::new(Stream::Ffi(FfiStreamWrapper(stream))).into()
}
