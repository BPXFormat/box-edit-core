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

use safer_ffi::prelude::*;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use crate::error::unwrap_result;

#[derive_ReprC]
#[repr(i32)]
pub enum CSeekFrom {
    Start = 0,
    Current,
    End
}

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
