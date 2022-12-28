#include "list_node.h"

ListNode::ListNode(int v) : val{ v }, next{} {}

ListNode::ListNode(int v, ListNode *n) : val{ v }, next{ n } {}

std::vector<int> ListNode::to_vec() const
{
    std::vector<int> v;
    for (auto *node{ this }; nullptr != node; node = node->next)
    {
        v.push_back(node->val);
    }

    return v;
}