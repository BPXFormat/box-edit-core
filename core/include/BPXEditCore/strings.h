#ifndef BPX_STRINGS_H
#define BPX_STRINGS_H

#include <BPXEditCore/common.h>

BPX_API bpx_section_handle_t bpx_strings_create(BPX_NONNULL bpx_container_t* container);

BPX_API bool bpx_strings_load(BPX_NONNULL const bpx_container_t* container, bpx_section_handle_t handle);

#endif
