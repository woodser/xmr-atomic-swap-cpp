#!/bin/sh

# build libmonero-cpp shared library
mkdir -p build && 
cd build && 
cmake .. && 
cmake --build . && 
make . -j$HOST_NCORES