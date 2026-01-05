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
    uint32_t compressionThreshold;
    uint32_t memoryThreshold;
} BPXOpenOptions;

typedef struct BPXCreateOptions {
    BPXContainerOptions options;
    uint32_t compressionThreshold;
    uint32_t memoryThreshold;
    BPXMainHeader mainHeader;
} BPXCreateOptions;

@class BPXStream;
@class BPXSection;

NS_ASSUME_NONNULL_BEGIN

@interface BPXContainer : NSObject

//TODO: Support mutating main header

@property(readonly) BPXMainHeader mainHeader;
@property(readonly) bpx_container_t* rawHandle;
@property(readonly) NSArray<BPXSection*>* sections;

-(instancetype)initFromStream:(BPXStream *)stream handle:(bpx_container_t*)handle;

-(BOOL)save:(NSError **)error;

-(void)addSection:(BPXSection*)section;

-(void)removeSection:(BPXSection*)section;

+(nullable instancetype)open:(BPXStream*)stream options:(BPXOpenOptions)options error:(NSError**)error;

+(nullable instancetype)open:(BPXStream*)stream error:(NSError**)error;

+(instancetype)create:(BPXStream*)stream options:(BPXCreateOptions)options;

+(instancetype)create:(BPXStream*)stream;

@end

NS_ASSUME_NONNULL_END
