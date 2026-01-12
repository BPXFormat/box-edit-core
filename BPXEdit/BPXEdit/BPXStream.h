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
#import <BPXEdit/DataStream.h>
#include <BPXEditCore/stream.h>
#import <BPXEdit/BPXContainer.h>

NS_ASSUME_NONNULL_BEGIN

@interface BPXStream : NSObject

@property(readonly) bpx_stream_t* rawHandle;

-(instancetype)initFromDataStream:(id<DataStream>)stream;
-(nullable instancetype)initFromFile:(NSString *)path create:(BOOL)create withError:(NSError **)error;

-(nullable BPXContainer*)openWithOptions:(BPXContainerOptions)options compressionThreshold:(uint32_t)compressionThreshold memoryThreshold:(uint32_t)memoryThreshold error:(NSError**)error;

-(nullable BPXContainer*)open:(NSError**)error;

-(BPXContainer*)createWithOptions:(BPXContainerOptions)options compressionThreshold:(uint32_t)compressionThreshold memoryThreshold:(uint32_t)memoryThreshold mainHeader:(BPXMainHeader)mainHeader;

-(BPXContainer*)create;

@end

NS_ASSUME_NONNULL_END
