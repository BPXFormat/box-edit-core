#include "bpx/stream.h"
#include "bpx/container.h"

int main(void) {
    bpx_create_options_t options;
    bpx_create_options_default(&options);
    bpx_stream_t stream = bpx_stream_create("./test.bpx");
    bpx_container_t container = bpx_container_create(stream, &options);
    if (container == NULL) {
        return 1;
    }
    bpx_container_save(container);
    return 0;
}
