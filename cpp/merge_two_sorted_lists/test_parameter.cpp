// This is an independent project of an individual developer. Dear PVS-Studio, please check it.
// PVS-Studio Static Code Analyzer for C, C++, C#, and Java: http://www.viva64.com
#include "test_parameter.h"

// C++ Standard Library
#include <algorithm>
#include <iterator>
#include <utility>

namespace Jade
{

Test_parameter::Test_parameter() = default;

Test_parameter::Test_parameter(std::string label,
    std::vector<int> parameter_1,
    std::vector<int> parameter_2,
    std::vector<int> expected_result) :
    m_label{ std::move(label) },
    m_parameter_1{ std::move(parameter_1) },
    m_parameter_2{ std::move(parameter_2) },
    m_expected_result{ std::move(expected_result) }
{
    // Empty.
}

std::string const &Test_parameter::get_label() const
{
    return m_label;
}

std::vector<int> const &Test_parameter::get_parameter_1() const
{
    return m_parameter_1;
}

std::vector<int> const &Test_parameter::get_parameter_2() const
{
    return m_parameter_2;
}

std::vector<int> const &Test_parameter::get_expected_result() const
{
    return m_expected_result;
}

} // Jade