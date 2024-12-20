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
        std::vector<int> valList;

        while (head != nullptr) {
            valList.push_back(head->val);
            head = head->next;
        }

        int size = valList.size();
        int maximum_sum = 0;

        for (int i = 0; i < size/2; i++) {
            maximum_sum = max(maximum_sum, valList[i]+valList[size-i-1]);
        }

        return maximum_sum;
    }
};