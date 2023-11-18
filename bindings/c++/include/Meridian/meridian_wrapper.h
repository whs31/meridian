//
// Created by whs31 on 11/18/2023.
//

#pragma once

#include <string>
#include <system_error>
#include <Libra/Global>
#include <Libra/Expected>

using std::string;
using std::string_view;

namespace Meridian
{
  namespace elevation
  {
    auto elevation(f64 latitude, f64 longitude) -> expected<f32, std::error_code>;
  } // elevation

  [[maybe_unused]] auto enable_logger() -> expected<void, std::error_code>;
  [[maybe_unused]] auto version() -> string;
  [[maybe_unused]] auto binaryDirectory() -> string;
} // Meridian
