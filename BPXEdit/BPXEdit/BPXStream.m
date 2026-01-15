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

#import "BPXEdit/BPXStream.h"
#import "BPXEdit/Util.h"

@implementation BPXStream {
    bpx_stream_t* _handle;
    bpx_virtual_stream_t _vtable;
    id<DataStream> _ds;
}

static ssize_t internal__bpx_stream_wrapper_read(void *userdata, bpx_bytes_t buffer) {
    id<DataStream> ds = (__bridge id<DataStream>)userdata;
    return [ds readTo:buffer.bytes withSize:buffer.len];
}

static ssize_t internal__bpx_stream_wrapper_write(void *userdata, bpx_bytes_const_t buffer) {
    id<DataStream> ds = (__bridge id<DataStream>)userdata;
    return [ds writeFrom:buffer.bytes withSize:buffer.len];
}

static ssize_t internal__bpx_stream_wrapper_seek(void *userdata, bpx_seek_from_t from, ssize_t pos) {
    id<DataStream> ds = (__bridge id<DataStream>)userdata;
    return [ds seekFrom:from withPos:pos];
}

static bool internal__bpx_stream_wrapper_flush(void *userdata) {
    id<DataStream> ds = (__bridge id<DataStream>)userdata;
    return [ds flush] == YES;
}

static void internal__bpx_stream_wrapper_release(void* userdata) {
    CFBridgingRelease(userdata);
}

-(bpx_stream_t*)rawHandle {
    return _handle;
}

-(instancetype)initFromDataStream:(id<DataStream>)stream {
    _ds = stream;
    _vtable.userdata = (void*)CFBridgingRetain(_ds);
    _vtable.release = &internal__bpx_stream_wrapper_release;
    _vtable.read = &internal__bpx_stream_wrapper_read;
    _vtable.write = &internal__bpx_stream_wrapper_write;
    _vtable.seek = &internal__bpx_stream_wrapper_seek;
    _vtable.flush = &internal__bpx_stream_wrapper_flush;
    _handle = bpx_stream_new(&_vtable);
    return self;
}

-(__nullable instancetype)initFromFile:(NSString *)path create:(BOOL)create withError:(NSError **)error {
    _ds = nil;
    const char *p = [path UTF8String];
    if (create) {
        _handle = bpx_stream_create(p);
    } else {
        _handle = bpx_stream_open(p);
    }
    if (_handle == NULL) {
        *error = BPXEditGetLastError();
        return nil;
    }
    return self;
}

-(void)dealloc {
    //TODO: Implement stream destruction
    
}

-(nullable BPXContainer*)openWithOptions:(BPXContainerOptions)options compressionThreshold:(uint32_t)compressionThreshold memoryThreshold:(uint32_t)memoryThreshold error:(NSError**)error {
    if (_handle == NULL)
        [NSException raise:NSObjectNotAvailableException format:@"Attempt to open a container from a dangling stream!"];
    bpx_open_options_t opts = {
            .memory_threshold = memoryThreshold,
            .compression_threshold = compressionThreshold,
            .flags = options
    };
    bpx_container_t* container = bpx_container_open(_handle, &opts);
    if (container == NULL) {
        *error = BPXEditGetLastError();
        return nil;
    }
    return [[BPXContainer alloc] initFromStream:self handle:container error:error];
}

-(nullable BPXContainer*)open:(NSError**)error {
    if (_handle == NULL)
        [NSException raise:NSObjectNotAvailableException format:@"Attempt to open a container from a dangling stream!"];
    bpx_open_options_t opts;
    bpx_open_options_default(&opts);
    bpx_container_t* container = bpx_container_open(_handle, &opts);
    if (container == NULL) {
        *error = BPXEditGetLastError();
        return nil;
    }
    return [[BPXContainer alloc] initFromStream:self handle:container error:error];
}

-(BPXContainer*)createWithOptions:(BPXContainerOptions)options compressionThreshold:(uint32_t)compressionThreshold memoryThreshold:(uint32_t)memoryThreshold mainHeader:(BPXMainHeader)mainHeader {
    if (_handle == NULL)
        [NSException raise:NSObjectNotAvailableException format:@"Attempt to create a container from a dangling stream!"];
    bpx_create_options_t opts = {
            .flags = options,
            .memory_threshold = memoryThreshold,
            .compression_threshold = compressionThreshold,
            .main_header = mainHeader
    };
    bpx_container_t* container = bpx_container_create(_handle, &opts);
    return [[BPXContainer alloc] initFromStream:self handle:container error:nil];
}

-(BPXContainer*)create {
    if (_handle == NULL)
        [NSException raise:NSObjectNotAvailableException format:@"Attempt to create a container from a dangling stream!"];
    bpx_create_options_t opts;
    bpx_create_options_default(&opts);
    bpx_container_t* container = bpx_container_create(_handle, &opts);
    return [[BPXContainer alloc] initFromStream:self handle:container error:nil];
}

@end
