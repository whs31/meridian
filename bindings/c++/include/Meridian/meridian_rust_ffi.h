#pragma once

extern "C"
{
  struct MeridianVersion
  {
    int major;
    int minor;
    int patch;
  };

  MeridianVersion meridian_version();
  const char* meridian_binary_directory();
  int meridian_elevation(double latitude, double longitude);
  bool meridian_enable_logger();
}