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
use icrate::Foundation::NSInteger;
use objc2::{extern_protocol, ProtocolType};
use objc2::rc::Id;
use objc2::runtime::{NSObjectProtocol, ProtocolObject};

extern_protocol! {
    pub unsafe trait DataStream: NSObjectProtocol {
        #[method(readTo:withSize:)]
        fn read_to(&self, buf: *mut c_void, size: usize) -> usize;

        #[method(writeFrom:withSize:)]
        fn write_from(&self, buf: *const c_void, size: usize) -> usize;

        #[method(seekFrom:withPos:)]
        fn seek_from(&self, from: NSInteger, pos: i64) -> u64;
    }

    unsafe impl ProtocolType for dyn DataStream {
    }
}

const SEEK_FROM_START: NSInteger = 0;
const SEEK_FROM_CURRENT: NSInteger = 1;
const SEEK_FROM_END: NSInteger = 2;

pub struct DataStreamPtr {
    underlying: Id<ProtocolObject<dyn DataStream>>
}

impl DataStreamPtr {
    pub fn from(data_stream: Id<ProtocolObject<dyn DataStream>>) -> Self {
        Self { underlying: data_stream }
    }
}

impl Read for DataStreamPtr {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let ptr = buf.as_mut_ptr() as *mut c_void;
        let len: usize = unsafe {
            //msg_send![self.underlying.as_ptr(), readTo: ptr withSize: buf.len()]
            self.underlying.read_to(ptr, buf.len())
        };
        Ok(len)
    }
}

impl Write for DataStreamPtr {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let ptr = buf.as_ptr() as *const c_void;
        let len: usize = unsafe {
            //msg_send![self.underlying.as_ptr(), writeFrom: ptr withSize: buf.len()]
            self.underlying.write_from(ptr, buf.len())
        };
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Seek for DataStreamPtr {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let new_pos: u64 = unsafe {
            match pos {
                SeekFrom::Start(v) => {
                    //msg_send![self.underlying.as_ptr(), seekFrom: SEEK_FROM_START withPos: v as i64]
                    self.underlying.seek_from(SEEK_FROM_START, v as _)
                }
                SeekFrom::End(v) => {
                    //msg_send![self.underlying.as_ptr(), seekFrom: SEEK_FROM_END withPos: v]
                    self.underlying.seek_from(SEEK_FROM_END, v)
                }
                SeekFrom::Current(v) => {
                    //msg_send![self.underlying.as_ptr(), seekFrom: SEEK_FROM_CURRENT withPos: v]
                    self.underlying.seek_from(SEEK_FROM_CURRENT, v)
                }
            }
        };
        Ok(new_pos)
    }
}
