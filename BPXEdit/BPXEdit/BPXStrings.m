// Copyright (c) 2026, BlockProject 3D
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

#import "BPXEdit/BPXStrings.h"
#import "BPXEdit/Util.h"

@implementation BPXStrings {
    NSMutableDictionary<NSNumber*, NSString*>* _dic;
}

-(instancetype)initFromSection:(BPXSection*)section {
    _section = section;
    _dic = [[NSMutableDictionary alloc] init];
    return self;
}

-(nullable NSString*)get:(NSInteger)pos error:(NSError**)error {
    NSString* s = [_dic objectForKey:[NSNumber numberWithInteger:pos]];
    if (s != nil)
        return s;
    if (![_section seekFrom:BPXSeekFromStart pos:pos error:error])
        return nil;
    NSData* data = [_section readUntil:'\0' maxSize:UINT32_MAX error:error];
    if (data == nil)
        return nil;
    s = [NSString stringWithUTF8String:data.bytes];
    if (s == nil) {
        *error = BPXEditUTF8Error();
        return nil;
    }
    [_dic setObject:s forKey:[NSNumber numberWithInteger:pos]];
    return s;
}

-(BOOL)put:(NSString*)data error:(NSError**)error {
    if (![_section seekFrom:BPXSeekFromEnd pos:0 error:error])
        return NO;
    _lastPos = _section.pos;
    NSData* s = [data dataUsingEncoding:NSUTF8StringEncoding];
    if (![_section writeAll:s error:error])
        return NO;
    [_dic setObject:data forKey:[NSNumber numberWithInteger:_lastPos]];
    return YES;
}

@end
