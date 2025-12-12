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
