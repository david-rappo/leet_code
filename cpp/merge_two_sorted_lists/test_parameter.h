#pragma once

// C++ Standard Library
#include <string>
#include <vector>

namespace Jade
{

class Test_parameter
{
public:
    Test_parameter();
    Test_parameter(std::string label,
        std::vector<int> parameter_1,
        std::vector<int> parameter_2,
        std::vector<int> expected_result);

    std::string const &get_label() const;
    std::vector<int> const &get_parameter_1() const;
    std::vector<int> const &get_parameter_2() const;
    std::vector<int> const &get_expected_result() const;

private:
    std::string m_label;
    std::vector<int> m_parameter_1;
    std::vector<int> m_parameter_2;
    std::vector<int> m_expected_result;
};

} // Jade