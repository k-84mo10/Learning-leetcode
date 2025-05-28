/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    int sumNumbers(TreeNode* root) {
        int sum = 0;

        stack<pair<TreeNode*, int>> s;
        s.push(make_pair(root, 0));
        
        while (!s.empty()) {
            auto [node, previous_sum] = s.top();
            s.pop();

            int current_sum = previous_sum * 10 + node->val;
            if (node->left == nullptr && node->right == nullptr) {
                sum += current_sum;
                continue;
            }
            if (node->left != nullptr) s.push(make_pair(node->left, current_sum));
            if (node->right != nullptr) s.push(make_pair(node->right, current_sum));
        }

        return sum;
    }
};