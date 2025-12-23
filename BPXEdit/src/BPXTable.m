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
        [value initFromRawHandle:bpx_table_row_get_value(_row, i)];
    }
    return self;
}

-(bpx_table_row_t*)rawHandle {
    return _row;
}

-setFree:(bool)free {
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
    [super dealloc];
}

@end

@implementation BPXTable {
    bpx_table_t* _table;
    BPXSection* _parent;
    BPXSection* _strings;
    BPXRow* _row;
}

-resetRow {
    if (_row == nil)
        _row = [[BPXRow alloc] init];
    [_row initFromRawHandle:_table];
}

-(instancetype)initFromSection:(BPXSection*)parent strings:(BPXSection*)strings rawHandle:(bpx_table_t*)table {
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
    ssize_t index = bpx_table_column_create(_table, name.UTF8String, type, len);
    if (index == -1) {
        *error = BPXEditGetLastError();
        return -1;
    }
    [self resetRow];
    return index;
}

-removeColumn:(NSInteger)index {
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
    [super dealloc];
}

@end
