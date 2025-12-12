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

#ifndef BPX_TREE_H
#define BPX_TREE_H

#include <BPXEditCore/common.h>

typedef void bpx_node_t;

typedef void bpx_value_t;

BPX_SLICE(node_children_list, const bpx_node_t*, nodes)

BPX_NONNULL BPX_API const char* bpx_node_get_name(const bpx_node_t* node);

BPX_NONNULL BPX_API const bpx_value_t* bpx_node_get_value(const bpx_node_t* node);

BPX_NULLABLE BPX_API const bpx_node_t* bpx_node_get_details(const bpx_node_t* node);

BPX_API bpx_node_children_list_t bpx_node_get_children(const bpx_node_t* node);

BPX_API bpx_value_type_t bpx_value_get_type(const bpx_value_t* value);

BPX_API bool bpx_value_is_null(const bpx_value_t* value);

BPX_API int8_t bpx_value_get_int8(const bpx_value_t* value);

BPX_API uint8_t bpx_value_get_uint8(const bpx_value_t* value);

BPX_API int16_t bpx_value_get_int16(const bpx_value_t* value);

BPX_API uint16_t bpx_value_get_uint16(const bpx_value_t* value);

BPX_API int32_t bpx_value_get_int32(const bpx_value_t* value);

BPX_API uint32_t bpx_value_get_uint32(const bpx_value_t* value);

BPX_API int64_t bpx_value_get_int64(const bpx_value_t* value);

BPX_API uint64_t bpx_value_get_uint64(const bpx_value_t* value);

BPX_API float32_t bpx_value_get_float(const bpx_value_t* value);

BPX_API float32_t bpx_value_get_double(const bpx_value_t* value);

BPX_API bool bpx_value_get_boolean(const bpx_value_t* value);

BPX_NULLABLE BPX_API const char* bpx_value_get_string(const bpx_value_t* value);

#endif
