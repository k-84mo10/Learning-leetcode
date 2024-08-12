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
    int traceNode(TreeNode *parent, int level)
    {
        int leftDepth = level;
        int rightDepth = level;

        if (parent->left != nullptr)
            leftDepth = traceNode(parent->left, level + 1);
        if (parent->right != nullptr)
            rightDepth = traceNode(parent->right, level + 1);

        return max(leftDepth, rightDepth);
    }

public:
    int maxDepth(TreeNode *root)
    {
        if (root == nullptr)
            return 0;
        int level = 1;
        return traceNode(root, level);
    }
};