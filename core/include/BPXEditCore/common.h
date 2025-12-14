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

#ifndef BPX_COMMON_H
#define BPX_COMMON_H

#include <stdbool.h>
#include <stdlib.h>
#include <sys/types.h>
#include <stdint.h>

#include <BPXEditCore/marker.h>

typedef BPX_NONNULL uint32_t bpx_section_handle_t;

typedef enum bpx_seek_from_e {
    BPX_SEEK_FROM_START = 0,
    BPX_SEEK_FROM_CURRENT,
    BPX_SEEK_FROM_END
} bpx_seek_from_t;

typedef struct bpx_section_header_s {
    uint64_t pointer;
    uint32_t csize;
    uint32_t size;
    uint32_t chksum;
    uint8_t type;
    uint8_t flags;
} bpx_section_header_t;

typedef struct bpx_section_info_s {
    bpx_section_header_t header;
    uint32_t index;
    bpx_section_handle_t handle;
} bpx_section_info_t;

typedef struct bpx_main_header_s {
    char signature[3];
    char type;
    uint32_t chksum;
    uint64_t file_size;
    uint32_t section_num;
    uint32_t version;
    uint8_t type_ext[16];
} bpx_main_header_t;

typedef enum bpx_value_type_e {
    BPX_VALUE_TYPE_NULL = 0,
    BPX_VALUE_TYPE_INT8,
    BPX_VALUE_TYPE_UINT8,
    BPX_VALUE_TYPE_INT16,
    BPX_VALUE_TYPE_UINT16,
    BPX_VALUE_TYPE_INT32,
    BPX_VALUE_TYPE_UINT32,
    BPX_VALUE_TYPE_INT64,
    BPX_VALUE_TYPE_UINT64,
    BPX_VALUE_TYPE_FLOAT,
    BPX_VALUE_TYPE_DOUBLE,
    BPX_VALUE_TYPE_BOOLEAN,
    BPX_VALUE_TYPE_STRING
} bpx_value_type_t;

typedef void bpx_container_t;

#define BPX_SLICE(name, type, value) typedef struct bpx_##name##_s { \
    BPX_NONNULL type* value;                                         \
    size_t len;                                                      \
} bpx_##name##_t;

BPX_SLICE(bytes, uint8_t, bytes);
BPX_SLICE(bytes_const, const uint8_t, bytes);
BPX_SLICE(section_list, const bpx_section_info_t, sections);

#define BPX_BYTES(ptr, len) (bpx_bytes_t){ ptr, len }
#define BPX_BYTES_CONST(ptr, len) (bpx_bytes_const_t){ ptr, len }

typedef float float32_t;
typedef double float64_t;

#endif
