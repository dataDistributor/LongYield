cmake_minimum_required(VERSION 3.5)

project(LongYieldGUI)

# Use C++11 standard
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find the Qt5 Widgets library
find_package(Qt5 REQUIRED COMPONENTS Widgets)

# Include directories: Qt and our FFI header.
include_directories(${Qt5Widgets_INCLUDE_DIRS} ${CMAKE_SOURCE_DIR}/include)

# Specify our source files.
set(SOURCES
    src/main.cpp
)

add_executable(${PROJECT_NAME} ${SOURCES})

# Specify the location of the Rust shared library.
# This assumes your Rust library is in ../target/debug relative to cpp/
set(RUST_LIB_DIR "${CMAKE_SOURCE_DIR}/../target/debug")

# Instead of linking with -llongyield, use the absolute path to the library.
target_link_libraries(${PROJECT_NAME} Qt5::Widgets "${RUST_LIB_DIR}/liblongyield.so")

# Set the runtime search path (RPATH) to the Rust library directory.
set_target_properties(${PROJECT_NAME} PROPERTIES
    INSTALL_RPATH "${RUST_LIB_DIR}"
)
