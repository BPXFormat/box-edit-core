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

#import <Foundation/Foundation.h>
#include <BPXEditCore/error.h>
#include <string.h>

NSError *BPXEditGetLastError() {
    int32_t code = bpx_get_last_error_code();
    const char *name = bpx_get_last_error_name();
    char buffer[1024];
    memset(buffer, 0, 1024);
    //Avoid removing the last null terminator.
    bpx_get_last_error_message((bpx_bytes_t){ .bytes = (uint8_t *)buffer, .len = 1023 });
    NSString *msg = [[NSString alloc] initWithCString:buffer encoding:NSUTF8StringEncoding];
    NSDictionary<NSErrorUserInfoKey, id> *dict = @{
        NSLocalizedDescriptionKey: msg,
        NSDebugDescriptionErrorKey: msg
    };
    NSString *nameObjc = [[NSString alloc] initWithCString:name encoding: NSUTF8StringEncoding];
    return [NSError errorWithDomain:nameObjc code:code userInfo:dict];
}
