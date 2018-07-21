cmake_minimum_required(VERSION 2.8)

project(courier)

# Use c++11
set (CMAKE_CXX_STANDARD 11)

# Set to highest warn level
if(MSVC)
  if(CMAKE_CXX_FLAGS MATCHES "/W[0-4]")
    string(REGEX REPLACE "/W[0-4]" "/W4" CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS}")
  else()
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /W4")
  endif()
elseif(CMAKE_COMPILER_IS_GNUCC OR CMAKE_COMPILER_IS_GNUCXX)
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wno-long-long -pedantic")
endif()

include_directories(courier_test PRIVATE courier/include)
include_directories(courier_test PRIVATE include)

add_subdirectory(cpr)
add_subdirectory(courier)

add_executable(courier_test main.cc)

target_link_libraries(courier_test libcourier)

install(TARGETS courier_test DESTINATION bin)
