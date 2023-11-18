//
// Created by whs31 on 11/18/2023.
//

#include "Meridian/meridian_wrapper.h"
#include "Meridian/meridian_rust_ffi.h"

[[maybe_unused]] auto Meridian::version() -> string
{
  auto v = ::meridian_version();
  return std::to_string(v.major) + "." + std::to_string(v.minor) + "." + std::to_string(v.patch);
}

[[maybe_unused]] auto Meridian::binaryDirectory() -> string { return { ::meridian_binary_directory() }; }

[[maybe_unused]] auto Meridian::enable_logger() -> expected<void, std::error_code>
{
  if(::meridian_enable_logger()) return {};
  else return unexpected(std::make_error_code(std::errc::already_connected));
}

auto Meridian::elevation::elevation(f64 latitude, f64 longitude) -> expected<f32, std::error_code>
{
  auto el = ::meridian_elevation(std::forward<decltype(latitude)>(latitude), std::forward<decltype(latitude)>(longitude));
  if(el == -404) return unexpected(std::make_error_code(std::errc::no_such_file_or_directory));
  else return static_cast<f32>(el);
}
