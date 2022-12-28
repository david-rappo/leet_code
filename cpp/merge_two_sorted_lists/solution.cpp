// This is an independent project of an individual developer. Dear PVS-Studio, please check it.
// PVS-Studio Static Code Analyzer for C, C++, C#, and Java: http://www.viva64.com
#include "solution.h"

#include <utility>

#include "list_node.h"

namespace
{

// Returns: new list, popped node
std::pair<ListNode *, ListNode *> pop_front(ListNode *list);

}

namespace Solution
{

ListNode *merge_two_lists(ListNode *list1, ListNode *list2)
{
    ListNode *head{};
    ListNode *tail{};
    std::pair<ListNode *, ListNode *> p{};
    while ((nullptr != list1) && (nullptr != list2))
    {
        if (list1->val < list2->val)
        {
            p = pop_front(list1);
            list1 = p.first;
        }
        else
        {
            p = pop_front(list2);
            list2 = p.first;
        }

        if (nullptr == head)
        {
            head = p.second;
            tail = head;
        }
        else
        {
            tail->next = p.second;
            tail = tail->next;
        }
    }

    auto *list{ (nullptr == list1) ? list2 : list1 };
    if (nullptr == list)
    {
        return head;
    }

    while (nullptr != list)
    {
        p = pop_front(list);
        list = p.first;
        if (nullptr == head)
        {
            head = p.second;
            tail = head;
        }
        else
        {
            tail->next = p.second;
            tail = tail->next;
        }
    }

    return head;
}

}

namespace
{

// Returns: new list, popped node
std::pair<ListNode *, ListNode *> pop_front(ListNode *list)
{
    if (nullptr == list)
    {
        return { nullptr, nullptr };
    }

    auto *popped_node{ list };
    auto *next{ list->next };
    popped_node->next = nullptr;
    return { next, popped_node };
}

}