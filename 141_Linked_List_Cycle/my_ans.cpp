/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    bool hasCycle(ListNode *head) {
        unordered_map<ListNode*, bool> node_list;
        
        while(head != NULL) {
            if (node_list.find(head) != node_list.end()) return true;
            node_list[head] = true;
            head = head->next;
        }
        
        return false;
    }
};