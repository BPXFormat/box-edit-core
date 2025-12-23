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

