cmake_minimum_required(VERSION 3.9.1)
project(voxelized_3d_cuda)


set(CMAKE_CXX_STANDARD 11)

#add_executable(voxelized_3d_cuda main.cpp cuda_test.cpp)

#target_link_libraries(voxelized_3d_cuda cudart)



set(CMAKE_CXX_FLAGS  -fpermissive)

include_directories(${CUDA_TOOLKIT_ROOT_DIR}/include)

#set_source_files_properties( main.cpp PROPERTIES CUDA_SOURCE_PROPERTY_FORMAT OBJ )

# Specify binary name and source file to build it from
add_library(
        voxelized3d STATIC
        main.cpp helper_math.h types.h FastNoise.cpp FastNoise.h)

install(TARGETS voxelized3d DESTINATION lib)