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
    ListNode* reverseBetween(ListNode* head, int left, int right) {
        ListNode* dummy_head = new ListNode();
        dummy_head->next = head;

        ListNode* lastNonReverse = dummy_head;
        for (int i=1; i<left; i++) {
            lastNonReverse = lastNonReverse->next;
        }

        ListNode* start = lastNonReverse->next;
        ListNode* then = start->next;

        for (int i=left; i<right; i++) {
            start->next = then->next;
            then->next = lastNonReverse->next;
            lastNonReverse->next = then;
            then = start->next;
        }

        ListNode* new_head = dummy_head->next;
        delete dummy_head;
        return new_head;
    }
};