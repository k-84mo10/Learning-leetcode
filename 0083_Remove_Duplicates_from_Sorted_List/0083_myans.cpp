/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution
{
public:
    ListNode *deleteDuplicates(ListNode *head)
    {
        ListNode *start = head;
        if (head == nullptr)
            return head;

        int duplicate_val = head->val;
        ListNode *newPastNode = head;

        while (head != nullptr)
        {
            if (head->val != duplicate_val)
            {
                duplicate_val = head->val;
                newPastNode->next = head;
                newPastNode = head;
            }
            head = head->next;
        }

        newPastNode->next = nullptr;
        return start;
    }
};