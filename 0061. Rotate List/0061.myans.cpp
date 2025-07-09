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
    ListNode* rotateRight(ListNode* head, int k) {
        if (!head) return nullptr;

        int n = 1;
        ListNode* current = head;
        while (current -> next != nullptr) {
            n += 1;
            current = current -> next;
        }

        k = k % n;
        if (k == 0) return head;

        ListNode* last = current;
        ListNode* new_current = head;
        for (int i = 0; i < n - k - 1; i++) {
            new_current = new_current -> next;
        }

        ListNode* new_head = new_current -> next;
        new_current -> next = nullptr;
        last -> next = head;
        head = new_head;

        return head;
    }
};