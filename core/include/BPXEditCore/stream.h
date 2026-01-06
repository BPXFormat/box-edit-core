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

#ifndef BPX_STREAM_H
#define BPX_STREAM_H

#include <BPXEditCore/common.h>

typedef void bpx_stream_t;

typedef struct bpx_virtual_stream_s {
    BPX_NONNULL void* userdata;
    void(*release)(BPX_NONNULL void* userdata);
    ssize_t(*read)(BPX_NONNULL void* userdata, bpx_bytes_t buffer);
    ssize_t(*write)(BPX_NONNULL void* userdata, bpx_bytes_const_t buffer);
    bool(*flush)(BPX_NONNULL void* userdata);
    ssize_t(*seek)(BPX_NONNULL void* userdata, bpx_seek_from_t from, ssize_t pos);
} bpx_virtual_stream_t;

BPX_NULLABLE BPX_API bpx_stream_t* bpx_stream_create(const char *path);
BPX_NULLABLE BPX_API bpx_stream_t* bpx_stream_open(const char *path);

BPX_NONNULL BPX_API bpx_stream_t* bpx_stream_new(BPX_NONNULL bpx_virtual_stream_t* vtable);

#endif
