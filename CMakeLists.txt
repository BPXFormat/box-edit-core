cmake_minimum_required(VERSION 3.20)
project(BPXEditCore)

add_subdirectory(BPXEdit)

add_executable(test-bpx-edit-core test/main.c)
target_link_libraries(test-bpx-edit-core PUBLIC ${CMAKE_SOURCE_DIR}/lib/target/BPXEditCore.xcframework)
target_link_libraries(test-bpx-edit-core PUBLIC BPXEdit)
