/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        if (root == nullptr) return nullptr;

        if (root == p) return p;
        if (root == q) return q;

        TreeNode* leftans = lowestCommonAncestor(root->left, p, q);
        TreeNode* rightans = lowestCommonAncestor(root->right, p, q);

        if (leftans == nullptr && rightans == nullptr) return nullptr;
        if (leftans != nullptr && rightans != nullptr) return root;
        if (leftans == nullptr && rightans != nullptr) return rightans;
        if (leftans != nullptr && rightans == nullptr) return leftans;

        return nullptr;
    }
};