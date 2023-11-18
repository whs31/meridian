cmake_minimum_required(VERSION 3.15)

project(Meridian
  VERSION 0.2.0
  DESCRIPTION "Library for working with elevations, geospatial data and cartography"
  HOMEPAGE_URL "com.radar-mms.uav"
  LANGUAGES C CXX
)

if(NOT CMAKE_CXX_STANDARD)
  set(CMAKE_CXX_STANDARD 20)
  set(CMAKE_CXX_STANDARD_REQUIRED ON)
endif()

set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON CACHE BOOL "Export all symbols")
set(CMAKE_SHARED_LIBRARY_ENABLE_EXPORTS ON)

option(BUILD_TESTS "Build tests" ON)

if(NOT WIN32)
  set(OPENSSL_USE_STATIC_LIBS TRUE)
  find_package(PkgConfig REQUIRED)
  pkg_search_module(OPENSSL REQUIRED openssl)
  if(OPENSSL_FOUND)
    include_directories(${OPENSSL_INCLUDE_DIRS})
    message(STATUS "Using OpenSSL ${OPENSSL_VERSION}")
  endif()
  find_package(Threads REQUIRED)
endif()

if(BUILD_TESTS)
  message(NOTICE "-- [Meridian] Tests enabled!")
  if(WIN32)
    include(FetchContent)
    FetchContent_Declare(
      googletest
      URL https://github.com/google/googletest/archive/03597a01ee50ed33e9dfd640b249b4be3799d395.zip
    )
    set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
    FetchContent_MakeAvailable(googletest)
  endif()

  find_package(GTest REQUIRED)
  enable_testing()
else()
  message(STATUS "[Meridian] Tests disabled")
endif()

add_subdirectory(libs/libra)
add_library(MeridianStaticRust STATIC IMPORTED GLOBAL)

if(WIN32)
  message(STATUS "[Meridian] Searching for .a in ${PROJECT_SOURCE_DIR}/plugins/Windows/x64/libmeridian.a")
  set_target_properties(MeridianStaticRust PROPERTIES IMPORTED_LOCATION
    ${PROJECT_SOURCE_DIR}/plugins/Windows/x64/libmeridian.a
  )
else()
  message(STATUS "[Meridian] Searching for .a in ${PROJECT_SOURCE_DIR}/plugins/Linux/x64/libmeridian.a")
  set_target_properties(MeridianStaticRust PROPERTIES IMPORTED_LOCATION
    ${PROJECT_SOURCE_DIR}/plugins/Linux/x64/libmeridian.a
  )
endif()

add_library(${PROJECT_NAME} STATIC)

target_sources(${PROJECT_NAME}
  PUBLIC
  include/Meridian/Meridian
  include/Meridian/meridian_wrapper.h
  include/Meridian/meridian_rust_ffi.h
  PRIVATE
  src/c++/meridian_wrapper.c++
)

if(WIN32)
  target_link_libraries(${PROJECT_NAME}
    PUBLIC
      Libra
      MeridianStaticRust
    PRIVATE
      wsock32
      ws2_32
      ntdll
      userenv
      bcrypt
      crypt32
      secur32
      ncrypt
  )
else()
  target_link_libraries(${PROJECT_NAME}
    PUBLIC
      Libra
      MeridianStaticRust
    PRIVATE
      ${OPENSSL_LIBRARIES}
      dl
      ssl
      crypto
      ${CMAKE_THREAD_LIBS_INIT}
  )
endif()

target_include_directories(${PROJECT_NAME}
  PUBLIC
    include
  PRIVATE
    src/c++
)

if(BUILD_TESTS)
  message(NOTICE "-- [Meridian] Building test executable")
  file(GLOB_RECURSE TEST_SOURCES tests/*)
  add_executable(${PROJECT_NAME}Test ${TEST_SOURCES})
  target_link_libraries(${PROJECT_NAME}Test ${PROJECT_NAME} ${GTEST_LIBRARIES} ${GTEST_MAIN_LIBRARIES})
  target_include_directories(${PROJECT_NAME}Test PRIVATE tests)
endif()

unset(BUILD_TESTS CACHE)