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

#import "BPXEdit/BPXSection.h"
#import "BPXEdit/Util.h"
#include <BPXEditCore/strings.h>
#import "BPXEdit/BPXTable.h"

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
    return self;
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
    return self;
}

-(instancetype)initInContainer:(BPXContainer*)parent type:(uint8_t)ty {
    bpx_section_options_t opts;
    bpx_section_options_default(&opts);
    opts.type = ty;
    bpx_section_handle_t handle = bpx_section_create(parent.rawHandle, &opts);
    [self _internal_init_new:parent handle:handle];
    return self;
}

-remove {
    [_parent removeSection:self];
}

+(instancetype)createStrings:(BPXContainer*)parent {
    bpx_section_handle_t handle = bpx_strings_create(parent.rawHandle);
    BPXSection* section = [[BPXSection alloc] init];
    [section _internal_init_new:parent handle:handle];
    return section;
}

+(nullable BPXTable*)createTable:(BPXContainer*)parent strings:(BPXSection*)strings name:(const NSString*)name error:(NSError**)error {
    bpx_table_t* table = bpx_table_create(parent.rawHandle, strings.rawHandle, name.UTF8String);
    if (table == NULL) {
        *error = BPXEditGetLastError();
        return nil;
    }
    BPXSection* section = [[BPXSection alloc] init];
    [section _internal_init_new:parent handle:bpx_table_handle(table)];
    return [[BPXTable alloc] initFromSection:section strings:strings rawHandle:table];
}

-(nullable BPXTable*)openTable:(BPXSection*)strings error:(NSError**)error {
    bpx_table_t* table = bpx_table_open(_parent.rawHandle, _handle, strings.rawHandle);
    if (table == NULL) {
        *error = BPXEditGetLastError();
        return nil;
    }
    return [[BPXTable alloc] initFromSection:self strings:strings rawHandle:table];
}

@end
