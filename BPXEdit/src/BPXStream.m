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

#import "BPXEdit/BPXStream.h"
#import "BPXEdit/Util.h"
#include "BPXEditCore/stream.h"

@implementation BPXStream {
    bpx_stream_t _stream;
    bpx_virtual_stream_t _vtable;
    id<DataStream> _ds;
}

static ssize_t internal__bpx_stream_wrapper_read(void *userdata, bpx_bytes_t buffer) {
    id<DataStream> ds = (id<DataStream>)userdata;
    return [ds readTo:buffer.bytes withSize:buffer.len];
}

static ssize_t internal__bpx_stream_wrapper_write(void *userdata, bpx_bytes_const_t buffer) {
    id<DataStream> ds = (id<DataStream>)userdata;
    return [ds writeFrom:buffer.bytes withSize:buffer.len];
}

static ssize_t internal__bpx_stream_wrapper_seek(void *userdata, bpx_seek_from_t from, ssize_t pos) {
    id<DataStream> ds = (id<DataStream>)userdata;
    return [ds seekFrom:from withPos:pos];
}

static bool internal__bpx_stream_wrapper_flush(void *userdata) {
    id<DataStream> ds = (id<DataStream>)userdata;
    return [ds flush] == YES;
}

- (instancetype)initFromDataStream:(id<DataStream>)stream {
    _ds = stream;
    _vtable.userdata = _ds;
    _vtable.read = &internal__bpx_stream_wrapper_read;
    _vtable.write = &internal__bpx_stream_wrapper_write;
    _vtable.seek = &internal__bpx_stream_wrapper_seek;
    _vtable.flush = &internal__bpx_stream_wrapper_flush;
    _stream = bpx_stream_new(&_vtable);
    return self;
}

-(__nullable instancetype)initFromFile:(NSString *)path create:(BOOL)create withError:(NSError **)error {
    _ds = nil;
    const char *p = [path UTF8String];
    if (create) {
        _stream = bpx_stream_create(p);
    } else {
        _stream = bpx_stream_open(p);
    }
    if (_stream == NULL) {
        *error = BPXEditGetLastError();
        return nil;
    }
    return self;
}

@end
