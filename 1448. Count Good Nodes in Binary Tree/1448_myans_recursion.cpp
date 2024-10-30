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
    int goodNodes(TreeNode* root) {
        return goodNodes(root, root->val);
    }

    int goodNodes(TreeNode* root, int current_max) {
        if (root == nullptr) return 0;

        int goodNode_num = (root->val >= current_max) ? 1 : 0;

        current_max = max(root->val, current_max);

        goodNode_num += goodNodes(root->left, current_max);
        goodNode_num += goodNodes(root->right, current_max);

        return goodNode_num;
    }
};