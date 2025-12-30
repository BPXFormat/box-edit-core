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

#import <BPXEdit/BPXContainer.h>
#import <BPXEdit/BPXSection.h>
#include <BPXEditCore/tree/value.h>

typedef NS_ENUM(int32_t, BPXValueType) {
    BPXValueTypeNull = BPX_VALUE_TYPE_NULL,
    BPXValueTypeInt8,
    BPXValueTypeUint8,
    BPXValueTypeInt16,
    BPXValueTypeUint16,
    BPXValueTypeInt32,
    BPXValueTypeUint32,
    BPXValueTypeInt64,
    BPXValueTypeUint64,
    BPXValueTypeFloat,
    BPXValueTypeDouble,
    BPXValueTypeBoolean,
    BPXValueTypeString
};

NS_ASSUME_NONNULL_BEGIN

@interface BPXValue : NSObject

@property(readonly) bpx_value_t* rawHandle;
@property(readonly) BPXValueType type;

@property(readonly) int8_t i8;
@property(readonly) int16_t i16;
@property(readonly) int32_t i32;
@property(readonly) int64_t i64;
@property(readonly) uint8_t u8;
@property(readonly) uint16_t u16;
@property(readonly) uint32_t u32;
@property(readonly) uint64_t u64;
@property(readonly) float f;
@property(readonly) double d;
@property(readonly) bool b;
@property(readonly, nullable) NSString* s;

-(instancetype)initFromRawHandle:(bpx_value_t*)value;

-(bool)isNull;

-(int64_t)toInt64;
-(uint64_t)toUint64;
-(double)toDouble;

@end

@interface BPXMutableValue : BPXValue

-(instancetype)initFromRawHandle:(bpx_value_t*)value;

-setNull;
-setInt8:(int8_t)v;
-setInt16:(int16_t)v;
-setInt32:(int32_t)v;
-setInt64:(int64_t)v;
-setUint8:(uint8_t)v;
-setUint16:(uint16_t)v;
-setUint32:(uint32_t)v;
-setUint64:(uint64_t)v;
-setFloat:(float)v;
-setDouble:(double)v;
-setBool:(bool)v;
-setString:(NSString*)v;

@end


NS_ASSUME_NONNULL_END

