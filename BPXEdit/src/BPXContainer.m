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
    for (size_t i = 0; i != list.count; ++i) {
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
    assert(list.sections[list.count - 1].handle == section.rawHandle);
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
