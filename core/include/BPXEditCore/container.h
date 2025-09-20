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

#ifndef BPX_CONTAINER_H
#define BPX_CONTAINER_H

#include <BPXEditCore/common.h>
#include <BPXEditCore/stream.h>

#define FLAG_IGNORE_CHECKSUM 0x1
#define FLAG_IGNORE_SIGNATURE 0x2
#define FLAG_IGNORE_VERSION 0x4
#define FLAG_REVERT_ON_SAVE_FAIL 0x8

typedef struct bpx_open_options_s {
    uint8_t flags;
    uint32_t memory_threshold;
} bpx_open_options_t;

typedef struct bpx_create_options_s {
    uint8_t flags;
    uint32_t memory_threshold;
    bpx_main_header_t main_header;
} bpx_create_options_t;

BPX_API void bpx_create_options_default(BPX_NONNULL bpx_create_options_t* options);
BPX_API void bpx_open_options_default(BPX_NONNULL bpx_open_options_t* options);

BPX_NONNULL BPX_API bpx_container_t* bpx_container_create(BPX_NONNULL bpx_stream_t *stream, BPX_NONNULL const bpx_create_options_t* options);
BPX_NULLABLE BPX_API bpx_container_t* bpx_container_open(BPX_NONNULL bpx_stream_t *stream, BPX_NONNULL const bpx_open_options_t* options);

BPX_NONNULL BPX_API const bpx_main_header_t* bpx_container_get_main_header(BPX_NONNULL const bpx_container_t* container);
BPX_API bpx_section_list_t bpx_container_get_sections(BPX_NONNULL const bpx_container_t* container);

BPX_API bool bpx_container_save(BPX_NONNULL bpx_container_t* container);

BPX_API void bpx_container_close(BPX_NONNULL bpx_container_t* container);

#endif
