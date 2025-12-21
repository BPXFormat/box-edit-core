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

#ifndef BPX_VALUE_H
#define BPX_VALUE_H

#include <BPXEditCore/common.h>

typedef void bpx_value_t;

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

BPX_API bpx_value_type_t bpx_value_get_type(BPX_NONNULL const bpx_value_t* value);

BPX_API bool bpx_value_is_null(BPX_NONNULL const bpx_value_t* value);

BPX_API int8_t bpx_value_get_int8(BPX_NONNULL const bpx_value_t* value);

BPX_API uint8_t bpx_value_get_uint8(BPX_NONNULL const bpx_value_t* value);

BPX_API int16_t bpx_value_get_int16(BPX_NONNULL const bpx_value_t* value);

BPX_API uint16_t bpx_value_get_uint16(BPX_NONNULL const bpx_value_t* value);

BPX_API int32_t bpx_value_get_int32(BPX_NONNULL const bpx_value_t* value);

BPX_API uint32_t bpx_value_get_uint32(BPX_NONNULL const bpx_value_t* value);

BPX_API int64_t bpx_value_get_int64(BPX_NONNULL const bpx_value_t* value);

BPX_API uint64_t bpx_value_get_uint64(BPX_NONNULL const bpx_value_t* value);

BPX_API float32_t bpx_value_get_float(BPX_NONNULL const bpx_value_t* value);

BPX_API float32_t bpx_value_get_double(BPX_NONNULL const bpx_value_t* value);

BPX_API bool bpx_value_get_boolean(BPX_NONNULL const bpx_value_t* value);

BPX_NULLABLE BPX_API const char* bpx_value_get_string(BPX_NONNULL const bpx_value_t* value);

#endif
