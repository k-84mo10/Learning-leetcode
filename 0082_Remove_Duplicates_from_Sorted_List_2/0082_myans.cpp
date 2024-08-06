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
        ListNode *dummy_start = new ListNode();
        ListNode *newPreviousNode = dummy_start;
        int duplicate_val = -101;

        while (head != nullptr)
        {
            if (head->val != duplicate_val)
            {
                if (head->next != nullptr && head->val == head->next->val)
                {
                    duplicate_val = head->val;
                }
                else
                {
                    newPreviousNode->next = head;
                    newPreviousNode = head;
                }
            }
            head = head->next;
        }

        newPreviousNode->next = nullptr;
        ListNode *newStart = dummy_start->next;
        delete dummy_start;
        return newStart;
    }
};