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

#import "BPXEdit/NSDataStream.h"

@implementation NSDataStream {
    NSData *_data;
    NSMutableData *_mutable;
    uint64_t _cursor;
}

-(instancetype)init:(NSData *)data {
    _data = data;
    _mutable = NULL;
    _cursor = 0;
    return self;
}

-(ssize_t)readTo:(void *)buffer withSize:(ssize_t)size {
    uint64_t start = _cursor;
    if (start >= _data.length)
        return 0;
    size_t remaining = _data.length - start;
    size = MIN(size, remaining);
    NSRange range = {
            .location = start,
            .length = size
    };
    [_data getBytes:buffer range:range];
    _cursor += size;
    return size;
}

-(ssize_t)writeFrom:(const void *)buffer withSize:(ssize_t)size {
    if (_mutable == NULL) {
        _mutable = [_data mutableCopy];
        _data = _mutable;
    }
    uint64_t start = _cursor;
    if (start >= _mutable.length)
        [_mutable increaseLengthBy:(start - _mutable.length) + size];
    size_t remaining = _mutable.length - start;
    size_t inRange = MIN(size, remaining);
    NSRange range = {
            .location = start,
            .length = inRange
    };
    [_mutable replaceBytesInRange:range withBytes:buffer];
    _cursor += inRange;
    if (inRange < size) {
        size_t posInBuffer = inRange;
        size_t remainingToWrite = size - inRange;
        [_mutable appendBytes:buffer + posInBuffer length:remainingToWrite];
    }
    return size;
}

-(ssize_t)seekFrom:(bpx_seek_from_t)from withPos:(ssize_t)pos {
    switch (from) {
        case BPX_SEEK_FROM_START:
            _cursor = (uint64_t)pos;
            break;
        case BPX_SEEK_FROM_CURRENT:
            _cursor += pos;
            break;
        case BPX_SEEK_FROM_END:
            _cursor = _data.length + pos;
            break;
    }
    return (ssize_t)_cursor;
}

-(BOOL)flush {
    return YES;
}

-(NSData *)data {
    return _data;
}

@end

