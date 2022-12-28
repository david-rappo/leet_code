#pragma once

#include <vector>

class ListNode
{
public:
    ListNode(int v);
    ListNode(int v, ListNode *n);

    std::vector<int> to_vec() const;

    int val{};
    ListNode *next{};
};