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
class Solution
{
    bool isValInRange(TreeNode *parent, TreeNode *minNode, TreeNode *maxNode)
    {
        if (parent == nullptr)
            return true;

        if (minNode != nullptr && minNode->val >= parent->val)
            return false;
        if (maxNode != nullptr && maxNode->val <= parent->val)
            return false;

        return isValInRange(parent->left, minNode, parent) && isValInRange(parent->right, parent, maxNode);
    }

public:
    bool isValidBST(TreeNode *root)
    {
        return isValInRange(root, nullptr, nullptr);
    }
};