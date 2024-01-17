cmake_minimum_required(VERSION 3.0.0)

project
(
    PROJECT_NAME 
    VERSION 1.0
    LANGUAGE CXX
)

# Setter navnet på exe filen til navnet og versionen
# eks. PROJECT1.0
set(EXECUTABLE_NAME "${PROJECT_NAME}_${VERSION}")

# Compilerer alle 
add_executable(
    ${EXECUTABLE_NAME}
    
    src/window/window.cpp
    ) 

# vet ikke om begge virker
target_link_directories()

# Skriv alle biblotekene manuelt
target_link_libraries()

# bare til /include/ dir
include_directories()