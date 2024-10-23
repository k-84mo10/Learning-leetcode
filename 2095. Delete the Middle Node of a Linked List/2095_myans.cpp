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
    ListNode* deleteMiddle(ListNode* head) {
        ListNode* first_tracer = head;
        ListNode* second_tracer = head;

        if (head == nullptr) return head;
        int n = 1;

        while (first_tracer->next != nullptr) {
            first_tracer = first_tracer->next;
            n++;
        }

        n = n/2;
        while (n > 1) {
            second_tracer = second_tracer->next;
            n--;
        }
        if (second_tracer->next == nullptr) return nullptr;
        second_tracer->next = second_tracer->next->next;

        return head;
    }
};
