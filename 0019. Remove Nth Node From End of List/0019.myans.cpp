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
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        int node_num = countNode(head);
        if (node_num - n == 0) return head->next;

        ListNode* currentNode = head;
        for (int i=0; i<node_num-n-1; i++) currentNode = currentNode->next;
        currentNode->next = currentNode->next->next;
        return head; 
    }

    int countNode(ListNode* head) {
        int count = 0;

        while (head != nullptr) {
            count += 1;
            head = head->next;
        } 

        return count;
    }
};