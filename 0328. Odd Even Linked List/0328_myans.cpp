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
    ListNode* oddEvenList(ListNode* head) {
        if(head == nullptr) return nullptr;

        ListNode* oddList = head;
        ListNode* oddListHead = oddList;

        if (head->next == nullptr) return oddList;
        ListNode* evenList = head->next;
        ListNode* evenListHead = evenList;
        head = head->next->next;
        bool is_odd = true;

        while (head != nullptr) {
            if (is_odd) {
                oddList->next = head;
                oddList = head;
                is_odd = false;
            } else {
                evenList->next = head;
                evenList = head;
                is_odd = true;
            }
            head = head->next;
        }

        oddList->next = evenListHead;
        evenList->next = nullptr;
        return oddListHead;
    }
};