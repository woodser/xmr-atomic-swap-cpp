list(INSERT CMAKE_MODULE_PATH 0 "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

cmake_minimum_required(VERSION 3.4.1)

#SET(CMAKE_C_COMPILER /path/to/c/compiler)
#SET(CMAKE_CXX_COMPILER /path/to/cpp/compiler)
SET(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fPIC -std=c++11 -F/Library/Frameworks -pthread")

project(XmrAtomicSwapCpp)

#set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/cmake/")

set(BUILD_LIBRARY ON)
set(BUILD_SCRATCHPAD ON)

####################
# Rust
####################

enable_language(Rust)
include(CMakeCargo)


####################
# Extra libraries
####################

if (APPLE)
  if (DEPENDS)
    list(APPEND EXTRA_LIBRARIES "-framework Foundation -framework IOKit -framework AppKit")
  else()
    find_library(COREFOUNDATION CoreFoundation)
    find_library(IOKIT IOKit)
    find_library(APPKIT AppKit)
    list(APPEND EXTRA_LIBRARIES ${IOKIT})
    list(APPEND EXTRA_LIBRARIES ${COREFOUNDATION})
    list(APPEND EXTRA_LIBRARIES ${APPKIT})
  endif()
endif()
if (WIN32)
  list(APPEND EXTRA_LIBRARIES setupapi)
endif()

message(STATUS EXTRA_LIBRARIES: ${EXTRA_LIBRARIES})

########################
# Build C++ library
########################

include_directories(src/main)

if (BUILD_LIBRARY)
    add_subdirectory(external/test-lib-rust)

    set(
        LIBRARY_SRC_FILES
        src/main/atomic_swap_utils.cpp
    )
    
    add_library(atomic_swap_utils SHARED ${LIBRARY_SRC_FILES})
    
    target_link_libraries(atomic_swap_utils
        ${EXTRA_LIBRARIES}
        test-lib-rust
    )
endif()

########################
# Build C++ Scratchpad
########################

if (BUILD_SCRATCHPAD)
    
    set(SCRATCHPAD_SRC_FILES src/test/scratchpad.cpp)
    
    add_executable(scratchpad ${LIBRARY_SRC_FILES} ${SCRATCHPAD_SRC_FILES})
    
    target_link_libraries(scratchpad
        ${EXTRA_LIBRARIES}
        test-lib-rust
    )
endif()