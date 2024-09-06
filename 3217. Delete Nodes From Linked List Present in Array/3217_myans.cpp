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
    ListNode *modifiedList(vector<int> &nums, ListNode *head)
    {
        unordered_set<int> remove_num;
        for (int &num : nums)
        {
            remove_num.insert(num);
        }

        ListNode *dummy = new ListNode();
        ListNode *start = dummy;
        ListNode *lastNode = start;
        dummy->next = head;

        if (head == nullptr)
            return nullptr;

        while (head != nullptr)
        {
            if (remove_num.find(head->val) == remove_num.end())
            {
                if (start == dummy)
                    start = head;
                lastNode->next = head;
                lastNode = head;
            }
            head = head->next;
        }

        lastNode->next = nullptr;
        delete dummy;
        return start;
    }
};