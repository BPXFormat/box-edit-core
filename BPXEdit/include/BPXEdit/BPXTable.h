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
