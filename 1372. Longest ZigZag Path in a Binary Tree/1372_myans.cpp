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
    int longestZigZag(TreeNode* root) {
        if (!root) return 0;
        return max(dfs(root->left, false, 0), dfs(root->right, true, 0));
    }

    int dfs(TreeNode* root, bool is_right, int currentLength) {
        if (!root) return currentLength;
        
        int leftLength = (is_right) ? dfs(root->left, false, currentLength+1) : dfs(root->left, false, 0);
        int rightLength = (is_right) ? dfs(root->right, true, 0) : dfs(root->right, true, currentLength+1);

        return max(leftLength, rightLength);
    }
};