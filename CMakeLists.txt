cmake_minimum_required(VERSION 3.28)

project
(
    PROJECT_NAME 
    VERSION 1.0
    LANGUAGE CXX
)

# Setter navnet på exe filen til navnet og versionen
# eks. PROJECT_1.0
set(EXECUTABLE_NAME "${PROJECT_NAME}_${VERSION}")

set
(
    SOURCES
    src/
    # Må linke til alle individuelt
)

# Hva skal bli til exe-en? 
add_executable(${EXECUTABLE_NAME} $(SOURCES)) 

# vet ikke om begge virker
target_link_directories
(
    ${EXECUTABLE_NAME} PRIVATE

    lib/sfml/
)

# Skriv alle biblotekene manuelt
target_link_libraries
(
    ${EXECUTABLE_NAME} PRIVATE

    lib/sfml/sfml-audio.lib
    lib/sfml/sfml-graphics.lib
    lib/sfml/sfml-network.lib
    lib/sfml/sfml-system.lib
    lib/sfml/sfml-window.lib
)

# bare til /include/ dir
include_directories(
    ${EXECUTABLE_NAME} PRIVATE

    include/sfml
)
