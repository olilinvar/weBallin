cmake_minimum_required(VERSION 3.12)

project(Project_Name VERSION 1.0)

set(SOURCES
    src/window/window.cpp    
)

set(EXECUTABLE_NAME "GAMING_${PROJECT_VERSION}")

add_executable(${EXECUTABLE_NAME} ${SOURCES})

# SFML bullshit GO!

set(SFML_DIR "C:/SFML-2.6.1/lib/cmake/SFML")

find_package(SFML 2.6 COMPONENTS graphics window system REQUIRED)

find_package(SFML COMPONENTS system window graphics REQUIRED)

# SFML bullshit END!