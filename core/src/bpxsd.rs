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
use crate::common::{try_with_section, Container};
use crate::error::unwrap_result;
use crate::tree::model::Node;

#[ffi_export]
pub fn bpxsd_read_from_bytes(buffer: c_slice::Ref<'_, u8>) -> Option<repr_c::Box<Node>> {
    let value = unwrap_result(bpx::sd::Value::read(buffer.as_slice(), 4))?;
    unwrap_result(Node::try_from(value.as_object()?).map(|v| Box::new(v).into()))
}

#[ffi_export]
pub fn bpxsd_read_from_section(container: &Container, handle: u32) -> Option<repr_c::Box<Node>> {
    let value = try_with_section(container, handle, |v| {
        bpx::sd::Value::read(v, 4)
    })?;
    unwrap_result(Node::try_from(value.as_object()?).map(|v| Box::new(v).into()))
}
