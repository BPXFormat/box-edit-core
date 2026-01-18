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

#import "BPXEdit/BPXTable.h"
#import "BPXEdit/Util.h"
#include <BPXEditCore/table/row.h>

@implementation BPXColumn

-(instancetype)__initFromIndex:(NSInteger)index name:(NSString*)name row:(BPXRow*)row {
    _index = index;
    _name = name;
    _type = (BPXValueType)bpx_value_get_type(bpx_table_row_get_value(row.rawHandle, index));
    return self;
}

@end

@implementation BPXRow {
    bpx_table_row_t* _row;
    NSMutableArray<BPXMutableValue*>* _values;
}

-(void)__setIndex:(NSInteger)index {
    _index = index;
}

-(instancetype)init {
    _row = NULL;
    _values = [[NSMutableArray alloc] init];
    return self;
}

-(instancetype)initFromRawHandle:(bpx_table_t*)table {
    if (_row != NULL)
        bpx_table_row_destroy(_row);
    _row = bpx_table_row_create(table);
    size_t columns = bpx_table_get_columns(table);
    for (size_t i = 0; i != columns; ++i) {
        BPXMutableValue* value;
        if (i < _values.count)
            value = _values[i];
        else {
            value = [BPXMutableValue alloc];
            [_values addObject:value];
        }
        (void)[value initFromRawHandle:bpx_table_row_get_value(_row, i)];
    }
    return self;
}

-(bpx_table_row_t*)rawHandle {
    return _row;
}

-(void)setFree:(bool)free {
    bpx_table_row_set_free(_row, free);
}

-(bool)isFree {
    return bpx_table_row_is_free(_row);
}

-(BPXMutableValue*)objectAtIndexedSubscript:(BPXColumn*)column {
    return _values[column.index];
}

-(void)dealloc {
    if (_row != NULL)
        bpx_table_row_destroy(_row);
}

@end

@implementation BPXTable {
    bpx_table_t* _table;
    BPXSection* _parent;
    BPXSection* _strings;
    BPXRow* _row;
}

-(void)resetRow {
    if (_row == nil)
        _row = [[BPXRow alloc] init];
    (void)[_row initFromRawHandle:_table];
}

-(nullable instancetype)initFromSection:(BPXSection*)parent strings:(BPXSection*)strings handle:(bpx_table_t*)table error:(NSError**)error {
    _table = table;
    _parent = parent;
    _strings = strings;
    _rowCount = 0;
    _row = nil;
    [self resetRow];
    _rowCount = bpx_table_get_row_count(_table, _row.rawHandle);
    if (![self updateRowCount:error])
        return nil;
    return self;
}

-(bpx_table_t*)rawHandle {
    return _table;
}

-(BPXSection*)section {
    return _parent;
}

-(BPXSection*)strings {
    return _strings;
}

-(NSString*)name {
    const char* name = bpx_table_get_name(_table);
    return [NSString stringWithUTF8String:name];
}

-(NSUInteger)rowSize {
    return bpx_table_get_row_size(_table);
}

-(NSUInteger)actualRowSize {
    return bpx_table_get_actual_row_size(_table);
}

-(nullable BPXColumn*)addColumn:(NSString*)name type:(BPXValueType)type len:(uint16_t)len error:(NSError**)error {
    ssize_t index = bpx_table_column_create(_table, name.UTF8String, (bpx_value_type_t)type, len);
    if (index == -1) {
        *error = BPXEditGetLastError();
        return nil;
    }
    [self resetRow];
    return [[BPXColumn alloc] __initFromIndex:index name:name row:_row];
}

-(void)removeColumn:(BPXColumn*)column {
    bpx_table_column_remove_at(_table, column.index);
    [self resetRow];
}

-(BOOL)updateRowCount:(NSError**)error {
    _rowCount = bpx_table_get_row_count(_table, _row.rawHandle);
    if (_rowCount == -1) {
        *error = BPXEditGetLastError();
        return NO;
    }
    return YES;
}

-(nullable BPXColumn*)columnForName:(NSString*)name error:(NSError**)error {
    ssize_t col = bpx_table_get_column_index(_table, name.UTF8String);
    if (col == -1) {
        *error = BPXEditGetLastError();
        return nil;
    }
    return [[BPXColumn alloc] __initFromIndex:col name:name row:_row];
}

-(BOOL)save:(NSError**)error {
    if (!bpx_table_save(_table)) {
        *error = BPXEditGetLastError();
        return NO;
    }
    return YES;
}

-(BPXRow*)newRow {
    return [[BPXRow alloc] initFromRawHandle:_table];
}

-(nullable BPXRow*)read:(NSInteger)index error:(NSError**)error {
    if (!bpx_table_read(_table, _row.rawHandle, index)) {
        *error = BPXEditGetLastError();
        return nil;
    }
    [_row __setIndex:index];
    return _row;
}

-(BOOL)write:(BPXRow*)row index:(NSInteger)index error:(NSError**)error {
    [row __setIndex:index];
    if (!bpx_table_write(_table, row.rawHandle, index)) {
        *error = BPXEditGetLastError();
        return NO;
    }
    return [self updateRowCount:error];
}

-(nullable BPXRow*)append:(BPXRow*)row error:(NSError**)error {
    NSInteger index = bpx_table_append(_table, row.rawHandle);
    if (index == -1) {
        *error = BPXEditGetLastError();
        return nil;
    }
    [row __setIndex:index];
    if (![self updateRowCount:error])
        return nil;
    return row;
}

-(void)dealloc {
    bpx_table_destroy(_table);
}

@end
