#ifndef BPX_TABLE_CORE_H
#define BPX_TABLE_CORE_H

#include <BPXEditCore/tree/value.h>

typedef void bpx_table_t;
typedef void bpx_table_row_t;

BPX_NULLABLE BPX_API bpx_table_t* bpx_table_create(BPX_NONNULL bpx_container_t* container, bpx_section_handle_t strings, BPX_NONNULL const char* name);

BPX_NULLABLE BPX_API bpx_table_t* bpx_table_open(BPX_NONNULL const bpx_container_t* container, bpx_section_handle_t handle, bpx_section_handle_t strings);

BPX_NONNULL BPX_API const char* bpx_table_get_name(BPX_NONNULL const bpx_table_t* table);

BPX_API bool bpx_table_save(BPX_NONNULL bpx_table_t* table);

BPX_API ssize_t bpx_table_column_create(BPX_NONNULL bpx_table_t* table, BPX_NONNULL const char* name, bpx_value_type_t ty, uint16_t len);

BPX_API void bpx_table_column_remove_at(BPX_NONNULL bpx_table_t* table, ssize_t index);

BPX_API size_t bpx_table_get_columns(BPX_NONNULL const bpx_table_t* table);

BPX_API size_t bpx_table_get_row_size(BPX_NONNULL const bpx_table_t* table);

BPX_API size_t bpx_table_get_actual_row_size(BPX_NONNULL const bpx_table_t* table);

BPX_API bpx_section_handle_t bpx_table_handle(BPX_NONNULL const bpx_table_t* table);

BPX_API bool bpx_table_read(BPX_NONNULL const bpx_table_t* table, BPX_NONNULL bpx_table_row_t* row, ssize_t index);

BPX_API bool bpx_table_write(BPX_NONNULL const bpx_table_t* table, BPX_NONNULL bpx_table_row_t* row, ssize_t index);

BPX_API ssize_t bpx_table_append(BPX_NONNULL const bpx_table_t* table, BPX_NONNULL bpx_table_row_t* row);

BPX_API ssize_t bpx_table_get_row_count(BPX_NONNULL const bpx_table_t* table, BPX_NONNULL const bpx_table_row_t* row);

BPX_API ssize_t bpx_table_get_column_index(BPX_NONNULL const bpx_table_t* table, BPX_NONNULL const char* name);

BPX_API void bpx_table_destroy(BPX_NONNULL bpx_table_t* table);

#endif
