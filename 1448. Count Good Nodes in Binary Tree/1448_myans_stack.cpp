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
        stack<pair<TreeNode*, int>> dfs;

        dfs.push({root, -10001});
        int goodNodes_num = 0;

        while (!dfs.empty()) {
            pair<TreeNode*, int> top = dfs.top();
            dfs.pop();
            if (top.first->val >= top.second) goodNodes_num++;
            if (top.first->left != nullptr) dfs.push({top.first->left, max(top.second, top.first->val)});
            if (top.first->right != nullptr) dfs.push({top.first->right, max(top.second, top.first->val)});
        } 

        return goodNodes_num;
    }

};