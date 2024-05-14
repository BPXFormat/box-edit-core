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

@implementation BPXContainer {
    BPXStream *_stream;
    bpx_container_t *_container;
}

-(instancetype)initFromRaw:(BPXStream *)stream container:(bpx_container_t *)ptr {
    _container = ptr;
    _stream = stream;
    return self;
}

-(void)dealloc {
    if (_container != NULL) {
        bpx_container_close(_container);
        _container = NULL;
    }
    [super dealloc];
}

-(BOOL)save:(NSError **)error {
    if (!bpx_container_save(_container)) {
        *error = BPXEditGetLastError();
        return NO;
    }
    return YES;
}

@end
