//
// Created by whs31 on 11/18/2023.
//

#include <gtest/gtest.h>
#include <Meridian/Meridian>

TEST(Basic, VersionString)
{
  EXPECT_STREQ("0.1.0", Meridian::version().c_str());
}

TEST(Elevation, ElevationAt)
{
  EXPECT_TRUE(Meridian::enable_logger().has_value());
  EXPECT_FLOAT_EQ(Meridian::elevation::elevation(60.0, 30.0).value(), 0.0f);
  EXPECT_FLOAT_EQ(Meridian::elevation::elevation(60.9, 30.9).value(), 3.0f);
  EXPECT_FLOAT_EQ(Meridian::elevation::elevation(60.5, 30.5).value(), 62.0f);
  EXPECT_FALSE(Meridian::elevation::elevation(0.0, 0.0).has_value());
}