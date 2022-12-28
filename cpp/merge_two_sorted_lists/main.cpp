// This is an independent project of an individual developer. Dear PVS-Studio, please check it.
// PVS-Studio Static Code Analyzer for C, C++, C#, and Java: http://www.viva64.com

// C++ Standard Library
#include <algorithm>
#include <cstddef>
#include <string>
#include <vector>

// Google Test
#include <gtest/gtest.h>

// Current
#include "list_node.h"
#include "list_node_guard.h"
#include "parameterized_test.h"
#include "solution.h"
#include "solution_test.h"
#include "test_parameter.h"

List_node_guard create_list(std::vector<int> const &v);

std::string get_test_name_suffix(testing::TestParamInfo<Jade::Test_parameter> const &parameter);

// Parameter #1 - TestFixtureName: A new instance of TestFixtureName is created for every test.
// Parameter #2 - TestName
TEST_P(ParameterizedTest, TestSolution)
{
    auto const &parameter{ GetParam() };
    auto list_1{ create_list(parameter.get_parameter_1()) };
    auto list_2{ create_list(parameter.get_parameter_2()) };
    auto *result = Solution::merge_two_lists(list_1.release(), list_2.release());
    List_node_guard guard{ result };
    std::vector<int> v;
    if (nullptr != guard.get_list_node())
    {
        v = guard.get_list_node()->to_vec();
    }

    EXPECT_EQ(v, parameter.get_expected_result());
}

std::vector<Jade::Test_parameter> const test_parameters = {
    Jade::Test_parameter{"TestCase1",
    {1, 2, 4},
    {1, 3, 4},
    {1, 1, 2, 3, 4, 4}},
    Jade::Test_parameter{"TestCase2",
    {},
    {},
    {}},
    Jade::Test_parameter{"TestCase3",
    {},
    {0},
    {0}},
    Jade::Test_parameter{"TestCase4",
    {0},
    {},
    {0}},
    Jade::Test_parameter{"TestCase5",
    {0, 1, 2, 3, 4, 5},
    {},
    {0, 1, 2, 3, 4, 5}},
    Jade::Test_parameter{"TestCase6",
    {},
    {0, 1, 2, 3, 4, 5},
    {0, 1, 2, 3, 4, 5}},
    Jade::Test_parameter{"TestCase7",
    {1, 1, 1},
    {2},
    {1, 1, 1, 2}},
    Jade::Test_parameter{"TestCase8",
    {2},
    {1, 1, 1},
    {1, 1, 1, 2}},
    Jade::Test_parameter{"TestCase9",
    {10, 11, 12},
    {13, 14, 15},
    {10, 11, 12, 13, 14, 15}},
    Jade::Test_parameter{"TestCase10",
    {1, 5, 100},
    {2, 3, 4},
    {1, 2, 3, 4, 5, 100}} };

INSTANTIATE_TEST_SUITE_P(Main,
    ParameterizedTest,
    testing::ValuesIn(test_parameters),
    get_test_name_suffix);

int main(int argc, char **argv)
{
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}

List_node_guard create_list(std::vector<int> const &v)
{
    ListNode *head{};
    ListNode *tail{};
    for (auto x : v)
    {
        if (nullptr == head)
        {
            head = new ListNode{ x };
            tail = head;
        }
        else
        {
            tail->next = new ListNode{ x };
            tail = tail->next;
        }
    }

    return List_node_guard{ head };
}

std::string get_test_name_suffix(testing::TestParamInfo<Jade::Test_parameter> const &parameter)
{
    return parameter.param.get_label();
}