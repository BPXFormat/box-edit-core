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

#import "BPXEdit/BPXContainer.h"
#import "BPXEdit/Util.h"
#import "BPXEdit/BPXStream.h"
#import "BPXEdit/BPXSection.h"

@implementation BPXContainer {
    BPXStream* _stream;
    bpx_container_t* _handle;
    NSMutableArray<BPXSection*>* _sections;
}

-(bpx_container_t*)rawHandle {
    return _handle;
}

-(NSArray<BPXSection*>*)sections {
    return _sections;
}

-(instancetype)initFromStream:(BPXStream *)stream handle:(bpx_container_t*)handle {
    _handle = handle;
    _stream = stream;
    _sections = [[NSMutableArray alloc] init];
    bpx_section_list_t list = bpx_container_get_sections(_handle);
    for (size_t i = 0; i != list.len; ++i) {
        BPXSection* obj = [[BPXSection alloc] initFromContainer:self infos:&list.sections[i]];
        [_sections addObject:obj];
    }
    return self;
}

-(void)dealloc {
    if (_handle != NULL) {
        bpx_container_close(_handle);
        _handle = NULL;
        _stream = nil;
    }
    [super dealloc];
}

-(BOOL)save:(NSError **)error {
    if (!bpx_container_save(_handle)) {
        *error = BPXEditGetLastError();
        return NO;
    }
    return YES;
}

-addSection:(BPXSection*)section {
    bpx_section_list_t list = bpx_container_get_sections(_handle);
    assert(list.sections[list.len - 1].handle == section.rawHandle);
    [_sections addObject:section];
}

-removeSection:(BPXSection*)section {
    bool flag = false;
    for (BPXSection* sec in _sections) {
        if (sec == section) {
            flag = true;
            break;
        }
    }
    assert(flag); //This ensures the section being removed is actually part of this container.
    bpx_section_remove(_handle, section.rawHandle);
    [_sections removeObject:section];
}

+(nullable instancetype)open:(BPXStream*)stream options:(BPXOpenOptions)options error:(NSError**)error {
    if (stream.rawHandle == NULL)
        [NSException raise:NSObjectNotAvailableException format:@"Attempt to open a container from a dangling stream!"];
    bpx_open_options_t opts = {
            .memory_threshold = options.memoryThreshold,
            .compression_threshold = options.compressionThreshold,
            .flags = options.options
    };
    bpx_container_t* container = bpx_container_open(stream.rawHandle, &opts);
    if (container == NULL) {
        *error = BPXEditGetLastError();
        return nil;
    }
    return [[BPXContainer alloc] initFromStream:stream handle:container];
}

+(nullable instancetype)open:(BPXStream*)stream error:(NSError**)error {
    if (stream.rawHandle == NULL)
        [NSException raise:NSObjectNotAvailableException format:@"Attempt to open a container from a dangling stream!"];
    bpx_open_options_t opts;
    bpx_open_options_default(&opts);
    bpx_container_t* container = bpx_container_open(stream.rawHandle, &opts);
    if (container == NULL) {
        *error = BPXEditGetLastError();
        return nil;
    }
    return [[BPXContainer alloc] initFromStream:stream handle:container];
}

+(instancetype)create:(BPXStream*)stream options:(BPXCreateOptions)options {
    if (stream.rawHandle == NULL)
        [NSException raise:NSObjectNotAvailableException format:@"Attempt to create a container from a dangling stream!"];
    bpx_create_options_t opts = {
            .flags = options.options,
            .memory_threshold = options.memoryThreshold,
            .compression_threshold = options.compressionThreshold,
            .main_header = options.mainHeader
    };
    bpx_container_t* container = bpx_container_create(stream.rawHandle, &opts);
    return [[BPXContainer alloc] initFromStream:stream handle:container];
}

+(instancetype)create:(BPXStream*)stream {
    if (stream.rawHandle == NULL)
        [NSException raise:NSObjectNotAvailableException format:@"Attempt to create a container from a dangling stream!"];
    bpx_create_options_t opts;
    bpx_create_options_default(&opts);
    bpx_container_t* container = bpx_container_create(stream.rawHandle, &opts);
    return [[BPXContainer alloc] initFromStream:stream handle:container];
}

@end
