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
#include <BPXEditCore/section.h>
#import <BPXEdit/BPXContainer.h>

@class BPXTable;

//TODO: Support mutation of section header

typedef NS_ENUM(int32_t, BPXSeekFrom) {
    BPXSeekFromStart = BPX_SEEK_FROM_START,
    BPXSeekFromCurrent = BPX_SEEK_FROM_CURRENT,
    BPXSeekFromEnd = BPX_SEEK_FROM_END
};

NS_ASSUME_NONNULL_BEGIN

typedef bpx_section_header_t BPXSectionHeader;

@interface BPXSection : NSObject

@property(readonly) BPXSectionHeader header;
@property(readonly) uint32_t index;
@property(readonly) bpx_section_handle_t rawHandle;
@property(readonly) NSInteger size;
@property(readonly) NSInteger bytesWritten;
@property(readonly) NSInteger pos;

-(nullable instancetype)initFromContainer:(BPXContainer*)parent infos:(const bpx_section_info_t*)infos error:(NSError**)error;

-(nullable instancetype)initFromContainer:(BPXContainer*)parent handle:(bpx_section_handle_t)handle error:(NSError**)error;

-(void)remove;

-(nullable BPXTable*)openTable:(BPXSection*)strings error:(NSError**)error;

-(nullable NSData*)read:(NSInteger)size error:(NSError**)error;

-(nullable NSData*)readExact:(NSInteger)size error:(NSError**)error;

-(nullable NSData*)readUntil:(Byte)byte maxSize:(NSInteger)size error:(NSError**)error;

-(BOOL)write:(NSData*)data error:(NSError**)error;

-(BOOL)writeAll:(NSData*)data error:(NSError**)error;

-(BOOL)seekFrom:(BPXSeekFrom)from pos:(NSInteger)pos error:(NSError**)error;

-(BOOL)shiftRight:(NSInteger)length error:(NSError**)error;

-(BOOL)shiftLeft:(NSInteger)length error:(NSError**)error;

@end

NS_ASSUME_NONNULL_END
