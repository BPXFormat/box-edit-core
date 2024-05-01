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

use std::cell::RefCell;
use std::error::Error;
use std::fmt::{Display, Formatter};
use safer_ffi::prelude::*;
use std::io::Cursor;
use std::io::Write;

pub trait IntoBPXError where Self: Sized + Error {
    const CODE: i32;
    const DOMAIN: &'static str;
}

impl IntoBPXError for bpx::core::error::Error {
    const CODE: i32 = 1;
    const DOMAIN: &'static str = "BPX";
}

impl IntoBPXError for bpx::sd::error::Error {
    const CODE: i32 = 2;
    const DOMAIN: &'static str = "BPXSD";
}

impl IntoBPXError for bpx::sd::error::TypeError {
    const CODE: i32 = 3;
    const DOMAIN: &'static str = "BPXSD TypeError";
}

impl IntoBPXError for std::io::Error {
    const CODE: i32 = 4;
    const DOMAIN: &'static str = "IO";
}

#[derive(Debug)]
pub enum RustError {
    Bpx(bpx::core::error::Error),
    Bpxsd(bpx::sd::error::Error),
    Type(bpx::sd::error::TypeError),
    Io(std::io::Error)
}

impl Display for RustError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RustError::Bpx(e) => write!(f, "BPX error: {}", e),
            RustError::Bpxsd(e) => write!(f, "BPXSD error: {}", e),
            RustError::Type(e) => write!(f, "Type error: {}", e),
            RustError::Io(e) => write!(f, "Io error: {}", e),
        }
    }
}

bpx::impl_err_conversion! (
    RustError {
        bpx::core::error::Error => Bpx,
        bpx::sd::error::Error => Bpxsd,
        bpx::sd::error::TypeError => Type,
        std::io::Error => Io
    }
);

pub struct BPXError {
    code: i32,
    error: Option<RustError>
}

impl BPXError {
    pub const fn none() -> BPXError {
        BPXError {
            code: -1,
            error: None
        }
    }
}

thread_local! {
    pub static LAST_ERROR: RefCell<BPXError> = RefCell::new(BPXError::none());
}

pub fn set_last_error<E: IntoBPXError + Into<RustError>>(error: E) {
    LAST_ERROR.replace(BPXError {
        code: E::CODE as _,
        error: Some(error.into())
    });
}

pub fn unwrap_result<T, E: IntoBPXError + Into<RustError>>(result: Result<T, E>) -> Option<T> {
    match result {
        Ok(v) => Some(v),
        Err(e) => {
            set_last_error(e);
            None
        }
    }
}

#[ffi_export]
pub fn bpx_get_last_error_code() -> i32 {
    LAST_ERROR.with_borrow(|e| e.code)
}

#[ffi_export]
pub fn bpx_get_last_error_message(out: c_slice::Mut<'_, u8>) {
    let mut buffer = Cursor::new(out.as_slice());
    LAST_ERROR.with_borrow(|e| {
        if let Some(e) = &e.error {
            let _ = write!(&mut buffer, "{}", e);
        }
        let _ = buffer.write(&[0]);
    })
}
