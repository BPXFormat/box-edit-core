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

