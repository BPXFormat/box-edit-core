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

@implementation BPXTable {
    bpx_table_t* _table;
    BPXSection* _parent;
    BPXSection* _strings;
}

-(instancetype)initFromSection:(BPXSection*)parent strings:(BPXSection*)strings rawHandle:(bpx_table_t*)table {
    _table = table;
    _parent = parent;
    _strings = strings;
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

@end
