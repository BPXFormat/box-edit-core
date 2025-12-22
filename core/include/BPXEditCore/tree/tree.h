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

#ifndef BPX_TREE_H
#define BPX_TREE_H

#include <BPXEditCore/tree/value.h>

typedef void bpx_node_t;

BPX_SLICE(node_children_list, const bpx_node_t*, nodes)

BPX_API void bpx_node_free(BPX_NONNULL bpx_node_t* node);

BPX_NONNULL BPX_API const char* bpx_node_get_name(BPX_NONNULL const bpx_node_t* node);

BPX_NONNULL BPX_API const bpx_value_t* bpx_node_get_value(BPX_NONNULL const bpx_node_t* node);

BPX_NULLABLE BPX_API const bpx_node_t* bpx_node_get_details(BPX_NONNULL const bpx_node_t* node);

BPX_API bpx_node_children_list_t bpx_node_get_children(BPX_NONNULL const bpx_node_t* node);

#endif
