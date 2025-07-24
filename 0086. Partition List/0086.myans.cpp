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
    ListNode* partition(ListNode* head, int x) {
        ListNode* small = new ListNode();
        ListNode* s_p = small;
        ListNode* large = new ListNode();
        ListNode* l_p = large;

        while (head != nullptr) {
            if (head->val < x) {
                s_p->next = head;
                s_p = s_p->next;
            } else {
                l_p->next = head;
                l_p = l_p->next;
            }
            head = head->next;
        }

        l_p->next = nullptr;
        s_p->next = large->next;

        ListNode* new_head = small->next;

        delete small;
        delete large;

        return new_head;
    }
};