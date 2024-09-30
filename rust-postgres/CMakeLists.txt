cmake_minimum_required(VERSION 3.23)

project(RustCxxPostgres CXX)

include(FetchContent)

set(CMAKE_EXPORT_COMPILE_COMMANDS true)
set(CMAKE_CXX_STANDARD 20)

# Fetch Corrosion to build Rust

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG master
)
FetchContent_MakeAvailable(Corrosion)

# Import the Rust crate

corrosion_import_crate(MANIFEST_PATH sqlx_lib/Cargo.toml)
corrosion_add_cxxbridge(cpp_rust_bridge CRATE sqlx_lib MANIFEST_PATH sqlx_lib FILES lib.rs)

# Add the C++ exe and link to the Rust bridge

add_executable(${PROJECT_NAME} cpp_exe/main.cpp)
target_link_libraries(${PROJECT_NAME} PRIVATE cpp_rust_bridge)

# Add system libraries

if (APPLE)

    find_library(CoreFoundation CoreFoundation)
    if (NOT CoreFoundation)
        message(FATAL_ERROR "CoreFoundation not found")
    endif()
    target_link_libraries(${PROJECT_NAME} PRIVATE ${CoreFoundation})

    find_library(SystemConfiguration SystemConfiguration)
    if (NOT SystemConfiguration)
        message(FATAL_ERROR "SystemConfiguration not found")
    endif()
    target_link_libraries(${PROJECT_NAME} PRIVATE ${SystemConfiguration})

elseif (WIN32)

    target_link_libraries(
        ${PROJECT_NAME} 
        PRIVATE 
        "bcrypt" "Secur32")

elseif (LINUX)

    # None needed at present

endif()