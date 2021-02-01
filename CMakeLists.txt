list(INSERT CMAKE_MODULE_PATH 0 "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

cmake_minimum_required(VERSION 3.4.1)

#SET(CMAKE_C_COMPILER /path/to/c/compiler)
#SET(CMAKE_CXX_COMPILER /path/to/cpp/compiler)
SET(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fPIC -std=c++11 -F/Library/Frameworks -pthread")

project(XmrAtomicSwapCpp)

set(BUILD_LIBRARY ON)
set(BUILD_SCRATCHPAD ON)

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

############
# Epee
############


############
# Boost
############

set(Boost_NO_BOOST_CMAKE 1)
set(Boost_USE_MULTITHREADED ON)
find_package(Boost 1.58 QUIET REQUIRED COMPONENTS chrono date_time filesystem program_options regex serialization wserialization system thread)
message(STATUS "Using Boost include dir at ${Boost_INCLUDE_DIR}")
include_directories(${Boost_INCLUDE_DIR})

########################
# Build C++ library
########################

include_directories(src/main)

set(
    LIBRARY_SRC_FILES
    src/main/xmr_atomic_swap_utils.cpp
)

if (BUILD_LIBRARY)
    add_library(xmr_atomic_swap_utils SHARED ${LIBRARY_SRC_FILES})
    
    target_link_libraries(xmr_atomic_swap_utils
        ${EXTRA_LIBRARIES}
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
    )
endif()