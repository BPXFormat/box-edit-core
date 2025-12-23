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
#import <BPXEdit/BPXValue.h>
#include <BPXEditCore/table/core.h>

NS_ASSUME_NONNULL_BEGIN

@interface BPXRow : NSObject

@property(readonly) bpx_table_row_t* rawHandle;

-setFree:(bool)free;

-(bool)isFree;

-(BPXValue*)objectAtIndexedSubscript:(NSInteger)index;

@end

@interface BPXTable : NSObject

@property(readonly) bpx_table_t* rawHandle;
@property(readonly) BPXSection* section;
@property(readonly) BPXSection* strings;

@property(readonly) NSString* name;
@property(readonly) size_t rowSize;
@property(readonly) size_t actualRowSize;

-(instancetype)initFromSection:(BPXSection*)parent strings:(BPXSection*)strings rawHandle:(bpx_table_t*)table;

-(NSInteger)addColumn:(NSString*)name type:(BPXValueType)type len:(uint16_t)len error:(NSError**)error;

-removeColumn:(NSInteger)index;

-(NSInteger)rowCount:(NSError**)error;

-(NSInteger)columnIndex:(NSString*)name error:(NSError**)error;

-(BPXRow*)getRow;

-(nullable BPXRow*)read:(NSInteger)index error:(NSError**)error;

-(bool)write:(BPXRow*)row index:(NSInteger)index error:(NSError**)error;

-(NSInteger)append:(BPXRow*)row error:(NSError**)error;

@end

NS_ASSUME_NONNULL_END
