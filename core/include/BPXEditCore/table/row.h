#ifndef BPX_TABLE_ROW_H
#define BPX_TABLE_ROW_H

#include <BPXEditCore/table/core.h>

BPX_NONNULL BPX_API bpx_table_row_t* bpx_table_row_create(BPX_NONNULL const bpx_table_t* table);

BPX_NONNULL BPX_API const bpx_value_t* bpx_table_row_get_value_const(BPX_NONNULL const bpx_table_row_t* row, size_t index);

BPX_NONNULL BPX_API bpx_value_t* bpx_table_row_get_value(BPX_NONNULL bpx_table_row_t* row, size_t index);

BPX_API bool bpx_table_row_is_free(BPX_NONNULL const bpx_table_row_t* row);

BPX_API void bpx_table_row_set_free(BPX_NONNULL bpx_table_row_t* row, bool free);

BPX_API void bpx_table_row_destroy(BPX_NONNULL bpx_table_row_t* row);

#endif
