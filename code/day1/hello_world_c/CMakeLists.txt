cmake_minimum_required(VERSION 3.16)

project(hello_world_c LANGUAGES C CXX)

add_executable(hello_c hello.c)
target_compile_features(hello_c PRIVATE c_std_11)
target_compile_options(hello_c PRIVATE -Wall -Wextra -pedantic)

add_executable(hello_cpp hello.cpp)
target_compile_features(hello_cpp PRIVATE cxx_std_17)
target_compile_options(hello_cpp PRIVATE -Wall -Wextra -pedantic)

