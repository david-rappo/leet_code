cmake_minimum_required(VERSION 3.16.3)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror")

project(merge_two_sorted_lists)

# SYSTEM - instructs the compiler to not generate warnings for these header files.
# - gtest/
include_directories(SYSTEM /usr/local/include)

# - libgtest
link_directories(/usr/local/lib)

# Required when building on Linux for PVS-Studio in cmake-gui
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(proj_srcs list_node.h list_node.cpp
              list_node_guard.h list_node_guard.cpp
              main.cpp
              parameterized_test.h parameterized_test.cpp
              solution.h solution.cpp
              solution_test.h solution_test.cpp
              test_parameter.h test_parameter.cpp
              utility.h utility.cpp)

add_executable(${PROJECT_NAME} ${proj_srcs})

target_link_libraries(${PROJECT_NAME} gtest)