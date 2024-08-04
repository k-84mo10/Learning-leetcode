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
    ListNode *reverseList(ListNode *head)
    {
        ListNode *nextNode;
        ListNode *pastNode = nullptr;

        while (head != nullptr)
        {
            if (head->next != nullptr)
            {
                nextNode = head->next;
                head->next = pastNode;
            }
            else
            {
                head->next = pastNode;
                break;
            }
            pastNode = head;
            head = nextNode;
        }

        return head;
    }
};