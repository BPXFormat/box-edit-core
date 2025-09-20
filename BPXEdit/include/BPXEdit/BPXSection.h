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

#import <Foundation/Foundation.h>
#include <BPXEditCore/section.h>
#import <BPXEdit/BPXContainer.h>

typedef NS_OPTIONS(uint8_t, BPXSectionOptions) {
    BPXSectionOptionsCompressXZ = FLAG_COMPRESS_XZ,
    BPXSectionOptionsCheckWeak = FLAG_CHECK_WEAK,
    BPXSectionOptionsCompressZLIB = FLAG_COMPRESS_ZLIB,
    BPXSectionOptionsCheckCRC32 = FLAG_CHECK_CRC32
};

NS_ASSUME_NONNULL_BEGIN

typedef bpx_section_header_t BPXSectionHeader;

@interface BPXSection : NSObject

@property(readonly) BPXSectionHeader header;
@property(readonly) uint32_t index;
@property(readonly) bpx_section_handle_t rawHandle;

-(instancetype)initFromContainer:(BPXContainer*)parent infos:(const bpx_section_info_t*)infos;

-(instancetype)initInContainer:(BPXContainer*)parent type:(uint8_t)ty options:(BPXSectionOptions)options compressionThreshold:(uint32_t)value;

-(instancetype)initInContainer:(BPXContainer*)parent type:(uint8_t)ty;

-remove;

@end

NS_ASSUME_NONNULL_END
