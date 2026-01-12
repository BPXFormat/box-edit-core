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

@implementation BPXRow {
    bpx_table_row_t* _row;
    NSMutableArray<BPXValue*>* _values;
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
        BPXValue* value;
        if (i < _values.count)
            value = _values[i];
        else {
            value = [BPXValue alloc];
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

-(BPXValue*)objectAtIndexedSubscript:(NSInteger)index {
    return _values[index];
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

-(instancetype)initFromSection:(BPXSection*)parent strings:(BPXSection*)strings handle:(bpx_table_t*)table {
    _table = table;
    _parent = parent;
    _strings = strings;
    _row = nil;
    [self resetRow];
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

-(size_t)rowSize {
    return bpx_table_get_row_size(_table);
}

-(size_t)actualRowSize {
    return bpx_table_get_actual_row_size(_table);
}

-(NSInteger)addColumn:(NSString*)name type:(BPXValueType)type len:(uint16_t)len error:(NSError**)error {
    ssize_t index = bpx_table_column_create(_table, name.UTF8String, (bpx_value_type_t)type, len);
    if (index == -1) {
        *error = BPXEditGetLastError();
        return -1;
    }
    [self resetRow];
    return index;
}

-(void)removeColumn:(NSInteger)index {
    bpx_table_column_remove_at(_table, index);
    [self resetRow];
}

-(NSInteger)rowCount:(NSError**)error {
    ssize_t len = bpx_table_get_row_count(_table, _row.rawHandle);
    if (len == -1) {
        *error = BPXEditGetLastError();
        return -1;
    }
    return len;
}

-(NSInteger)columnIndex:(NSString*)name error:(NSError**)error {
    ssize_t col = bpx_table_get_column_index(_table, name.UTF8String);
    if (col == -1) {
        *error = BPXEditGetLastError();
        return -1;
    }
    return col;
}

-(BPXRow*)getRow {
    return _row;
}

-(nullable BPXRow*)read:(NSInteger)index error:(NSError**)error {
    if (!bpx_table_read(_table, _row.rawHandle, index)) {
        *error = BPXEditGetLastError();
        return nil;
    }
    return _row;
}

-(bool)write:(BPXRow*)row index:(NSInteger)index error:(NSError**)error {
    if (!bpx_table_write(_table, row.rawHandle, index)) {
        *error = BPXEditGetLastError();
        return false;
    }
    return true;
}

-(NSInteger)append:(BPXRow*)row error:(NSError**)error {
    NSInteger index = bpx_table_append(_table, row.rawHandle);
    if (index == -1) {
        *error = BPXEditGetLastError();
        return -1;
    }
    return index;
}

-(void)dealloc {
    bpx_table_destroy(_table);
}

@end
