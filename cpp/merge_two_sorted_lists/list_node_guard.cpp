#include "list_node_guard.h"

#include "list_node.h"

List_node_guard::List_node_guard(ListNode *list_node) : m_list_node{ list_node } {}

List_node_guard::~List_node_guard()
{
    auto *node{ m_list_node };
    while (nullptr != node)
    {
        auto *next{ node->next };
        delete node;
        node = next;
    }
}

ListNode *List_node_guard::release()
{
    auto *list{ m_list_node };
    m_list_node = nullptr;
    return list;
}

ListNode const *List_node_guard::get_list_node() const
{
    return m_list_node;
}