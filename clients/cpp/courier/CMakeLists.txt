cmake_minimum_required(VERSION 2.8)

add_library(libcourier STATIC src/client.cc src/types.cc)

target_include_directories(libcourier PRIVATE ${CPR_INCLUDE_DIRS})

target_link_libraries(libcourier ${CPR_LIBRARIES})

install(TARGETS libcourier DESTINATION lib)
install(FILES include/courier/client.h include/courier/types.h DESTINATION include/courier)
