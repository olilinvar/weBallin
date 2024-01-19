cmake_minimum_required(VERSION 3.28)

project(PROJECT_NAME VERSION 1.0 LANGUAGES CXX)

set(EXECUTABLE_NAME "${PROJECT_NAME}_${VERSION}")


add_executable(${EXECUTABLE_NAME}) 

target_sources(${EXECUTABLE_NAME} PRIVATE 
    src/window/window.cpp
)

target_include_directories(${EXECUTABLE_NAME} PRIVATE include/sfml)

target_compile_features(${EXECUTABLE_NAME} PRIVATE cxx_std_17)

target_link_libraries(${EXECUTABLE_NAME} PRIVATE lib/sfml/sfml-graphics lib/sfml/sfml-window)

#C:\Users\xxxxxx\Onedrive\Dokumenter\weBallin\lib\sfml\sfml-system.lib





#target_link_directories(
#    ${EXECUTABLE_NAME} PRIVATE
#
#    lib/sfml/sfml-window.lib
#)
#
#target_link_libraries(
#    ${EXECUTABLE_NAME} PRIVATE
#
#    lib/sfml/sfml-audio.lib
#    lib/sfml/sfml-graphics.lib
#    lib/sfml/sfml-network.lib
#    lib/sfml/sfml-system.lib
#    lib/sfml/sfml-window.lib
#)
#include_directories(
#    ${EXECUTABLE_NAME} PRIVATE
#
#    include/sfml
#)