cmake_minimum_required(VERSION 3.20)
project(BPXEdit OBJC)

set(SRC
        include/BPXEdit/BPXEdit.h
        include/BPXEdit/DataStream.h
        include/BPXEdit/NSDataStream.h
        include/BPXEdit/BPXStream.h
        include/BPXEdit/Util.h
        src/NSDataStream.m
        src/BPXStream.m
        src/Util.m
)

add_library(BPXEdit SHARED ${SRC})
target_include_directories(BPXEdit PUBLIC include)
target_link_libraries(BPXEdit PUBLIC ${CMAKE_SOURCE_DIR}/lib/target/BPXEditCore.xcframework)
target_link_libraries(BPXEdit PUBLIC "-framework Foundation")
set_target_properties(BPXEdit PROPERTIES
        FRAMEWORK TRUE
        FRAMEWORK_VERSION A
        MACOSX_FRAMEWORK_IDENTIFIER com.github.BPXEdit
        #MACOSX_FRAMEWORK_INFO_PLIST Info.plist
        # "current version" in semantic format in Mach-O binary file
        VERSION 1.0.0
        # "compatibility version" in semantic format in Mach-O binary file
        SOVERSION 1.0.0
        PUBLIC_HEADER BPXEdit.h
)
