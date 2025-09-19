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

#ifndef BPX_SECTION_H
#define BPX_SECTION_H

#include <BPXEditCore/common.h>

#define FLAG_COMPRESS_XZ 0x2
#define FLAG_CHECK_WEAK 0x8
#define FLAG_COMPRESS_ZLIB 0x1
#define FLAG_CHECK_CRC32 0x4

#define SECTION_TYPE_STRING 0xFF
#define SECTION_TYPE_SD 0xFE

typedef struct bpx_section_options_s {
    uint8_t type;
    uint8_t flags;
    uint32_t compression_threshold;
} bpx_section_options_t;

bpx_section_handle_t bpx_section_create(BPX_NONNULL bpx_container_t* container, BPX_NONNULL const bpx_section_options_t* options);

void bpx_section_remove(BPX_NONNULL bpx_container_t* container, bpx_section_handle_t section);

ssize_t bpx_section_size(BPX_NONNULL const bpx_container_t* container, bpx_section_handle_t section);

ssize_t bpx_section_seek(BPX_NONNULL const bpx_container_t* container, bpx_section_handle_t section, bpx_seek_from_t from, ssize_t pos);

ssize_t bpx_section_read(BPX_NONNULL const bpx_container_t* container, bpx_section_handle_t section, bpx_bytes_t buffer);

ssize_t bpx_section_write(BPX_NONNULL const bpx_container_t* container, bpx_section_handle_t section, bpx_bytes_const_t buffer);

bool bpx_section_shift_left(BPX_NONNULL const bpx_container_t* container, bpx_section_handle_t section, size_t length);

bool bpx_section_shift_right(BPX_NONNULL const bpx_container_t* container, bpx_section_handle_t section, size_t length);

#endif
