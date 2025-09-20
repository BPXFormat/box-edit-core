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

#ifndef BPX_STREAM_H
#define BPX_STREAM_H

#include <BPXEditCore/common.h>

typedef void bpx_stream_t;

typedef struct bpx_virtual_stream_s {
    void* userdata;
    ssize_t(*read)(void* userdata, bpx_bytes_t buffer);
    ssize_t(*write)(void* userdata, bpx_bytes_const_t buffer);
    bool(*flush)(void* userdata);
    ssize_t(*seek)(void* userdata, bpx_seek_from_t from, ssize_t pos);
} bpx_virtual_stream_t;

BPX_NULLABLE BPX_API bpx_stream_t* bpx_stream_create(const char *path);
BPX_NULLABLE BPX_API bpx_stream_t* bpx_stream_open(const char *path);

BPX_NONNULL BPX_API bpx_stream_t* bpx_stream_new(BPX_NONNULL bpx_virtual_stream_t* virtual);

#endif
