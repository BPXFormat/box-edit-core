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
    ssize_t compression_threshold;
} bpx_section_options_t;

BPX_API void bpx_section_options_default(BPX_NONNULL bpx_section_options_t* options);

BPX_API bpx_section_handle_t bpx_section_create(BPX_NONNULL bpx_container_t* container, BPX_NONNULL const bpx_section_options_t* options);

BPX_API void bpx_section_remove(BPX_NONNULL bpx_container_t* container, bpx_section_handle_t section);

BPX_API ssize_t bpx_section_size(BPX_NONNULL const bpx_container_t* container, bpx_section_handle_t section);

BPX_API ssize_t bpx_section_seek(BPX_NONNULL const bpx_container_t* container, bpx_section_handle_t section, bpx_seek_from_t from, ssize_t pos);

BPX_API ssize_t bpx_section_read(BPX_NONNULL const bpx_container_t* container, bpx_section_handle_t section, bpx_bytes_t buffer);

BPX_API bool bpx_section_read_exact(BPX_NONNULL const bpx_container_t* container, bpx_section_handle_t section, bpx_bytes_t buffer);

BPX_API ssize_t bpx_section_write(BPX_NONNULL const bpx_container_t* container, bpx_section_handle_t section, bpx_bytes_const_t buffer);

BPX_API bool bpx_section_write_all(BPX_NONNULL const bpx_container_t* container, bpx_section_handle_t section, bpx_bytes_const_t buffer);

BPX_API bool bpx_section_shift_left(BPX_NONNULL const bpx_container_t* container, bpx_section_handle_t section, size_t length);

BPX_API bool bpx_section_shift_right(BPX_NONNULL const bpx_container_t* container, bpx_section_handle_t section, size_t length);

#endif
