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

#import <Foundation/Foundation.h>
#include <BPXEditCore/error.h>
#include <string.h>

NSError* BPXEditGetLastError(void) {
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

NSError* BPXEditUTF8Error(void) {
    NSDictionary<NSErrorUserInfoKey, id> *dict = @{
        NSLocalizedDescriptionKey: @"invalid utf-8 string",
        NSDebugDescriptionErrorKey: @"invalid utf-8 string"
    };
    return [NSError errorWithDomain:@"BPXEdit" code:128 userInfo:dict];
}

NSError* BPXEditSectionNotFoundError(void) {
    NSDictionary<NSErrorUserInfoKey, id> *dict = @{
        NSLocalizedDescriptionKey: @"section not found",
        NSDebugDescriptionErrorKey: @"section not found"
    };
    return [NSError errorWithDomain:@"BPXEdit" code:128 userInfo:dict];
}
