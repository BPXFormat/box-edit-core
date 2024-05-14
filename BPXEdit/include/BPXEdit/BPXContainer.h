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
#include <BPXEditCore/container.h>

typedef NS_OPTIONS(uint8_t, BPXContainerOptions) {
    BPXContainerOptionsIgnoreChecksum = FLAG_IGNORE_CHECKSUM,
    BPXContainerOptionsIgnoreSignature = FLAG_IGNORE_SIGNATURE,
    BPXContainerOptionsIgnoreVersion = FLAG_IGNORE_VERSION,
    BPXContainerOptionsRevertOnSaveFail = FLAG_REVERT_ON_SAVE_FAIL
};

typedef bpx_main_header_t BPXMainHeader;

typedef struct BPXOpenOptions {
    BPXContainerOptions options;
    uint32_t memoryThreshold;
} BPXOpenOptions;

typedef struct BPXCreateOptions {
    BPXContainerOptions options;
    uint32_t memoryThreshold;
    BPXMainHeader mainHeader;
} BPXCreateOptions;

@class BPXStream;

NS_ASSUME_NONNULL_BEGIN

@interface BPXContainer : NSObject

@property (readonly) BPXMainHeader mainHeader;

-(instancetype)initFromRaw:(BPXStream *)stream container:(bpx_container_t *)ptr;

-(BOOL)save:(NSError **)error;

@end

NS_ASSUME_NONNULL_END
