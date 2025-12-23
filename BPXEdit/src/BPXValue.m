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

#import "BPXEdit/BPXValue.h"

@implementation BPXValue {
    bpx_value_t* _value;
}

-(instancetype)initFromRawHandle:(bpx_value_t*)value {
    _value = value;
    return self;
}

-(bpx_value_t*)rawHandle {
    return _value;
}

-(BPXValueType)type {
    return bpx_value_get_type(_value);
}

-(bool)isNull {
    return bpx_value_is_null(_value);
}

-(int8_t) i8 {
    return bpx_value_get_int8(_value);
}

-(int16_t) i16 {
    return bpx_value_get_int16(_value);
}

-(int32_t) i32 {
    return bpx_value_get_int32(_value);
}

-(int64_t) i64 {
    return bpx_value_get_int64(_value);
}

-(uint8_t) u8 {
    return bpx_value_get_uint8(_value);
}

-(uint16_t) u16 {
    return bpx_value_get_uint16(_value);
}

-(uint32_t) u32 {
    return bpx_value_get_uint32(_value);
}

-(uint64_t) u64 {
    return bpx_value_get_uint64(_value);
}

-(float) f {
    return bpx_value_get_float(_value);
}

-(double) d {
    return bpx_value_get_double(_value);
}

-(bool) b {
    return bpx_value_get_boolean(_value);
}

-(nullable NSString*) s {
    const char* s = bpx_value_get_string(_value);
    if (s == NULL)
        return nil;
    return [NSString stringWithUTF8String:s];
}

-(int64_t)toInt64 {
    switch (self.type) {
        case BPXValueTypeNull:
            return 0;
        case BPXValueTypeInt8:
            return (int64_t)self.i8;
        case BPXValueTypeUint8:
            return (int64_t)self.u8;
        case BPXValueTypeInt16:
            return (int64_t)self.i16;
        case BPXValueTypeUint16:
            return (int64_t)self.u16;
        case BPXValueTypeInt32:
            return (int64_t)self.i32;
        case BPXValueTypeUint32:
            return (int64_t)self.u32;
        case BPXValueTypeInt64:
            return self.i64;
        case BPXValueTypeUint64:
            return (int64_t)self.u64;
        case BPXValueTypeFloat:
            return (int64_t)self.f;
        case BPXValueTypeDouble:
            return (int64_t)self.d;
        case BPXValueTypeBoolean:
            return self.b ? 1 : 0;
        case BPXValueTypeString:
            return 0;
    }
}

-(uint64_t)toUint64 {
    switch (self.type) {
        case BPXValueTypeNull:
            return 0;
        case BPXValueTypeInt8:
            return (uint64_t)self.i8;
        case BPXValueTypeUint8:
            return (uint64_t)self.u8;
        case BPXValueTypeInt16:
            return (uint64_t)self.i16;
        case BPXValueTypeUint16:
            return (uint64_t)self.u16;
        case BPXValueTypeInt32:
            return (uint64_t)self.i32;
        case BPXValueTypeUint32:
            return (uint64_t)self.u32;
        case BPXValueTypeInt64:
            return (uint64_t)self.i64;
        case BPXValueTypeUint64:
            return self.u64;
        case BPXValueTypeFloat:
            return (uint64_t)self.f;
        case BPXValueTypeDouble:
            return (uint64_t)self.d;
        case BPXValueTypeBoolean:
            return self.b ? 1 : 0;
        case BPXValueTypeString:
            return 0;
    }
}

-(double)toDouble {
    switch (self.type) {
        case BPXValueTypeNull:
            return 0;
        case BPXValueTypeInt8:
            return (double)self.i8;
        case BPXValueTypeUint8:
            return (double)self.u8;
        case BPXValueTypeInt16:
            return (double)self.i16;
        case BPXValueTypeUint16:
            return (double)self.u16;
        case BPXValueTypeInt32:
            return (double)self.i32;
        case BPXValueTypeUint32:
            return (double)self.u32;
        case BPXValueTypeInt64:
            return (double)self.i64;
        case BPXValueTypeUint64:
            return (double)self.u64;
        case BPXValueTypeFloat:
            return (double)self.f;
        case BPXValueTypeDouble:
            return self.d;
        case BPXValueTypeBoolean:
            return self.b ? 1.0 : 0.0;
        case BPXValueTypeString:
            return 0;
    }
}

@end

@implementation BPXMutableValue {
}

-(instancetype)initFromRawHandle:(bpx_value_t*)value {
    return [super initFromRawHandle:value];
}

-setNull {
    bpx_value_set_null(self.rawHandle);
}

-setInt8:(int8_t)v {
    bpx_value_set_int8(self.rawHandle, v);
}

-setInt16:(int16_t)v {
    bpx_value_set_int16(self.rawHandle, v);
}

-setInt32:(int32_t)v {
    bpx_value_set_int32(self.rawHandle, v);
}

-setInt64:(int64_t)v {
    bpx_value_set_int64(self.rawHandle, v);
}

-setUint8:(uint8_t)v {
    bpx_value_set_uint8(self.rawHandle, v);
}

-setUint16:(uint16_t)v {
    bpx_value_set_uint16(self.rawHandle, v);
}

-setUint32:(uint32_t)v {
    bpx_value_set_uint32(self.rawHandle, v);
}

-setUint64:(uint64_t)v {
    bpx_value_set_uint64(self.rawHandle, v);
}

-setFloat:(float)v {
    bpx_value_set_float(self.rawHandle, v);
}

-setDouble:(double)v {
    bpx_value_set_double(self.rawHandle, v);
}

-setBool:(bool)v {
    bpx_value_set_boolean(self.rawHandle, v);
}

-setString:(NSString*)v {
    bpx_value_set_string(self.rawHandle, v.UTF8String);
}

@end
