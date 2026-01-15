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
    NSMutableData* _data;
}

-(BPXSectionHeader)header {
    return _header;
}

-(uint32_t)index {
    return _index;
}

-(bpx_section_handle_t)rawHandle {
    return _handle;
}

-(bool)updateSize:(NSError**)error {
    // This is only used when creating sections, when the size will obviously be 0.
    if (error == nil) {
        _size = 0;
        return true;
    }
    _size = bpx_section_size(_parent.rawHandle, _handle);
    if (_size == -1) {
        *error = BPXEditGetLastError();
        return false;
    }
    return true;
}

-(nullable instancetype)initFromContainer:(BPXContainer*)parent infos:(const bpx_section_info_t*)infos error:(NSError**)error {
    assert(infos != NULL);
    _parent = parent;
    _handle = infos->handle;
    _index = infos->index;
    _header = infos->header;
    _data = nil;
    _pos = 0;
    _bytesWritten = 0;
    _size = 0;
    if (![self updateSize:error])
        return nil;
    return self;
}

-(nullable instancetype)initFromContainer:(BPXContainer*)parent handle:(bpx_section_handle_t)handle error:(NSError**)error {
    bpx_section_list_t list = bpx_container_get_sections(parent.rawHandle);
    const bpx_section_info_t* infos = NULL;
    for (size_t i = list.len; i != 0; --i) {
        infos = &list.sections[i - 1];
        if (infos->handle == handle)
            break;
    }
    assert(infos != NULL);
    if ([self initFromContainer:parent infos:infos error:error] == nil)
        return nil;
    [parent addSection:self];
    return self;
}

-(void)remove {
    [_parent removeSection:self];
}

-(nullable BPXTable*)openTable:(BPXSection*)strings error:(NSError**)error {
    bpx_table_t* table = bpx_table_open(_parent.rawHandle, _handle, strings.rawHandle);
    if (table == NULL) {
        *error = BPXEditGetLastError();
        return nil;
    }
    return [[BPXTable alloc] initFromSection:self strings:strings handle:table error:error];
}

-(nullable NSData*)read:(NSInteger)size error:(NSError**)error {
    [_data setLength:size];
    bpx_bytes_t bytes = {
        .bytes = _data.mutableBytes,
        .len = _data.length
    };
    NSInteger res = bpx_section_read(_parent.rawHandle, _handle, bytes);
    if (res < 0) {
        *error = BPXEditGetLastError();
        return nil;
    }
    [_data setLength:res];
    _pos += res;
    return _data;
}

-(nullable NSData*)readUntil:(Byte)byte maxSize:(NSInteger)size error:(NSError**)error {
    uint8_t data = 0;
    bpx_bytes_t bytes = {
        .bytes = &data,
        .len = 1
    };
    [_data setLength:0];
    NSInteger res = bpx_section_read(_parent.rawHandle, _handle, bytes);
    if (res < 0) {
        *error = BPXEditGetLastError();
        return nil;
    }
    [_data appendBytes:&data length:1];
    while (data != byte && size > 0) {
        NSInteger res = bpx_section_read(_parent.rawHandle, _handle, bytes);
        if (res < 0) {
            *error = BPXEditGetLastError();
            return nil;
        }
        [_data appendBytes:&data length:1];
        --size;
    }
    return _data;
}

-(BOOL)write:(NSData*)data error:(NSError**)error {
    bpx_bytes_const_t bytes = {
        .bytes = data.bytes,
        .len = data.length
    };
    NSInteger res = bpx_section_write(_parent.rawHandle, _handle, bytes);
    if (res < 0) {
        *error = BPXEditGetLastError();
        return NO;
    }
    if (![self updateSize:error])
        return NO;
    _bytesWritten += res;
    _pos += res;
    return YES;
}

-(BOOL)seekFrom:(BPXSeekFrom)from pos:(NSInteger)pos error:(NSError**)error {
    NSInteger res = bpx_section_seek(_parent.rawHandle, _handle, (bpx_seek_from_t)from, pos);
    if (res < 0) {
        *error = BPXEditGetLastError();
        return NO;
    }
    if (![self updateSize:error])
        return NO;
    _pos = res;
    _bytesWritten = 0;
    return YES;
}

@end
