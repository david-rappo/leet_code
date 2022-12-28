#pragma once

class ListNode;

class List_node_guard
{
public:
    explicit List_node_guard(ListNode *list_node);
    List_node_guard(List_node_guard const &) = delete;
    ~List_node_guard();

    List_node_guard &operator=(List_node_guard const &) = delete;

    ListNode *release();
    ListNode const *get_list_node() const;

private:
    ListNode *m_list_node{};
};