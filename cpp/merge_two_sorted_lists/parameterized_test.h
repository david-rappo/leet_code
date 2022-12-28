#pragma once

// Google Test
#include <gtest/gtest.h>

#include "test_parameter.h"

// Test fixture. Google's standard naming convention for test fixtures is
// Test{FixtureName}.
class ParameterizedTest : public ::testing::TestWithParam<Jade::Test_parameter>
{
};