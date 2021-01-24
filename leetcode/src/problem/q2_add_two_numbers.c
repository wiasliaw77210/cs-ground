#include <assert.h>
#include <malloc.h>

struct ListNode
{
    int val;
    struct ListNode *next;
};

void push(struct ListNode **head, int new_value)
{
    struct ListNode **indirect = head;

    struct ListNode *new_node = malloc(sizeof(struct ListNode));
    new_node->val = new_value;
    new_node->next = NULL;

    assert(new_node);
    while (*indirect)
        indirect = &(*indirect)->next;
    *indirect = new_node;
}

struct ListNode *addTwoNumbers(struct ListNode *l1, struct ListNode *l2)
{
    struct ListNode *ptr = NULL;
    int carry = 0;
    for (struct ListNode *a = l1, *b = l2; a || b; a = a ? a->next : NULL, b = b ? b->next : NULL)
    {
        int v1 = a ? a->val : 0;
        int v2 = b ? b->val : 0;
        push(&ptr, (v1 + v2 + carry) % 10);
        carry = ((v1 + v2 + carry) >= 10) ? 1 : 0;
    }
    if (carry == 1)
    {
        push(&ptr, carry);
    }
    return ptr;
}
