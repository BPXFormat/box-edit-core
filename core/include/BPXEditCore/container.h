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
    uint32_t compression_threshold;
} bpx_open_options_t;

typedef struct bpx_create_options_s {
    uint8_t flags;
    uint32_t memory_threshold;
    uint32_t compression_threshold;
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
