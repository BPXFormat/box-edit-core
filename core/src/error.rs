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

use std::cell::RefCell;
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{Display, Formatter};
use safer_ffi::prelude::*;
use std::io::Cursor;
use std::io::Write;
use crate::tree;

pub trait IntoBPXError where Self: Sized + Error {
    const CODE: i32;
    const DOMAIN: &'static CStr;
}

impl IntoBPXError for bpx::core::error::Error {
    const CODE: i32 = 1;
    const DOMAIN: &'static CStr = c"BPX";
}

impl IntoBPXError for bpx::sd::error::Error {
    const CODE: i32 = 2;
    const DOMAIN: &'static CStr = c"BPXSD";
}

impl IntoBPXError for bpx::sd::error::TypeError {
    const CODE: i32 = 3;
    const DOMAIN: &'static CStr = c"BPXSD TypeError";
}

impl IntoBPXError for std::io::Error {
    const CODE: i32 = 4;
    const DOMAIN: &'static CStr = c"IO";
}

impl IntoBPXError for tree::util::Error {
    const CODE: i32 = 5;
    const DOMAIN: &'static CStr = c"Tree";
}

impl IntoBPXError for bpx::table::error::Error {
    const CODE: i32 = 6;
    const DOMAIN: &'static CStr = c"BPX Table";
}

impl IntoBPXError for bpx::strings::Error {
    const CODE: i32 = 7;
    const DOMAIN: &'static CStr = c"BPX String";
}

#[derive(Debug)]
pub enum RustError {
    Bpx(bpx::core::error::Error),
    Bpxsd(bpx::sd::error::Error),
    Type(bpx::sd::error::TypeError),
    Io(std::io::Error),
    Tree(tree::util::Error),
    Table(bpx::table::error::Error),
    String(bpx::strings::Error)
}

impl Display for RustError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RustError::Bpx(e) => write!(f, "BPX error: {}", e),
            RustError::Bpxsd(e) => write!(f, "BPXSD error: {}", e),
            RustError::Type(e) => write!(f, "Type error: {}", e),
            RustError::Io(e) => write!(f, "Io error: {}", e),
            RustError::Tree(e) => write!(f, "Tree error: {}", e),
            RustError::Table(e) => write!(f, "Table error: {}", e),
            RustError::String(e) => write!(f, "String error: {}", e),
        }
    }
}

bpx::impl_err_conversion! (
    RustError {
        bpx::core::error::Error => Bpx,
        bpx::sd::error::Error => Bpxsd,
        bpx::sd::error::TypeError => Type,
        std::io::Error => Io,
        tree::util::Error => Tree,
        bpx::table::error::Error => Table,
        bpx::strings::Error => String
    }
);

pub struct BPXError {
    code: i32,
    domain: &'static CStr,
    error: Option<RustError>
}

impl BPXError {
    pub const fn none() -> BPXError {
        BPXError {
            code: -1,
            domain: c"",
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
        domain: E::DOMAIN,
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
pub fn bpx_get_last_error_name() -> char_p::Ref<'static> {
    LAST_ERROR.with_borrow(|e| e.domain.into())
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
