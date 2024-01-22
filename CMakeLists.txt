cmake_minimum_required(VERSION 3.16)
project(SFMLOpenGLProject)

# Set the path to SFML
set(SFML_DIR "SFML-2.6.1-windows-vc17-64-bit/SFML-2.6.1/lib/cmake/SFML")

# Find SFML components
find_package(SFML COMPONENTS system window graphics REQUIRED)

# Find OpenGL
find_package(OpenGL REQUIRED)

# Add your source files
set(SOURCES
    src/window/window.cpp
    # Add your other source files here
)

# Create the executable
add_executable(${PROJECT_NAME} ${SOURCES})

# Link SFML libraries
target_link_libraries(${PROJECT_NAME} PRIVATE sfml-system sfml-window sfml-graphics)

# Link OpenGL
target_link_libraries(${PROJECT_NAME} PRIVATE OpenGL::GL)

# Set C++ standard
target_compile_features(${PROJECT_NAME} PRIVATE cxx_std_17)
