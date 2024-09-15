cmake_minimum_required(VERSION 3.26)
project(AoC_2023)

set(CMAKE_CXX_STANDARD 14)

include_directories(.)
include_directories(src)

add_executable(AoC_2023
        src/Day4.c
        src/Day7.cpp)
