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
class Solution {
public:
    int pairSum(ListNode* head) {
        ListNode* slow = head;
        ListNode* fast = head;
        ListNode* start = head;

        while (fast != nullptr) {
            slow = slow->next;
            fast = fast->next->next;
        }

        ListNode* next = nullptr;
        ListNode* prev = nullptr;
        while (slow != nullptr) {
            next = slow->next;
            slow->next = prev;
            prev = slow;
            slow = next;
        }

        int maximum = 0;
        while (prev != nullptr) {
            maximum = max(maximum, start->val+prev->val);
            start = start->next;
            prev = prev->next;
        }
        return maximum;
    }
};