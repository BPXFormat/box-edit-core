cmake_minimum_required(VERSION 3.20)
project(BPXEditCore)

include(FetchContent)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.4 # Optionally specify a commit hash, version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)
corrosion_import_crate(MANIFEST_PATH ${CMAKE_SOURCE_DIR}/lib/Cargo.toml)
target_include_directories(bpx-edit-core INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/lib/include)

add_custom_target(bpx-edit-core-src
    SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/lib/include ${CMAKE_CURRENT_SOURCE_DIR}/lib/src ${CMAKE_CURRENT_SOURCE_DIR}/lib/Cargo.toml
)

add_executable(test-bpx-edit-core test/main.c)
target_link_libraries(test-bpx-edit-core PUBLIC bpx-edit-core)
