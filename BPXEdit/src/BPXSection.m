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

#import "BPXEdit/BPXSection.h"
#import "BPXEdit/Util.h"

@implementation BPXSection {
    bpx_section_handle_t _handle;
    BPXSectionHeader _header;
    uint32_t _index;
    BPXContainer* _parent;
}

-(BPXSectionHeader)header {
    return _header;
}

-(uint32_t)index {
    return _index;
}

-(ssize_t)sizeWithError:(NSError**)error {
    ssize_t res = bpx_section_size(_parent.rawHandle, _handle);
    if (res == -1) {
        *error = BPXEditGetLastError();
        return -1;
    }
    return res;
}

-(bpx_section_handle_t)rawHandle {
    return _handle;
}

-(instancetype)initFromContainer:(BPXContainer*)parent infos:(const bpx_section_info_t*)infos {
    assert(infos != NULL);
    _parent = parent;
    _handle = infos->handle;
    _index = infos->index;
    _header = infos->header;
}

-_internal_init_new:(BPXContainer*)parent handle:(bpx_section_handle_t)handle {
    bpx_section_list_t list = bpx_container_get_sections(parent.rawHandle);
    const bpx_section_info_t* infos = NULL;
    for (size_t i = list.len; i != 0; --i) {
        infos = &list.sections[i - 1];
        if (infos->handle == handle)
            break;
    }
    [self initFromContainer:parent infos:infos];
    [parent addSection:self];
}

-(instancetype)initInContainer:(BPXContainer*)parent type:(uint8_t)ty options:(BPXSectionOptions)options compressionThreshold:(uint32_t)value {
    bpx_section_options_t opts;
    opts.compression_threshold = value;
    opts.flags = options;
    opts.type = ty;
    bpx_section_handle_t handle = bpx_section_create(parent.rawHandle, &opts);
    [self _internal_init_new:parent handle:handle];
}

-(instancetype)initInContainer:(BPXContainer*)parent type:(uint8_t)ty {
    bpx_section_options_t opts;
    bpx_section_options_default(&opts);
    opts.type = ty;
    bpx_section_handle_t handle = bpx_section_create(parent.rawHandle, &opts);
    [self _internal_init_new:parent handle:handle];
}

-remove {
    [_parent removeSection:self];
}

@end
