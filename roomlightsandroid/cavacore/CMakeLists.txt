cmake_minimum_required(VERSION 3.13.0)
project(cavacore-jni)

# Add the cavacore library from ../cava
add_subdirectory(../cava cava_build)

# Ourself
add_library(cavacore-jni SHARED cavacore_jni.c)

# Link with cavacore
target_link_libraries(cavacore-jni PRIVATE cavacore fftw3)

target_include_directories(cavacore-jni PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/../cava
	"${FFTW_DIR}/jni/fftw3/api/"
)

target_link_directories(cavacore-jni PRIVATE "${FFTW_DIR}/obj/local/${CMAKE_ANDROID_ARCH_ABI}/")

# JNI
# find_package(JNI REQUIRED)