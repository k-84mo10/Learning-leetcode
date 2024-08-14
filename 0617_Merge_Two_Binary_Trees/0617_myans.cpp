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
    void sumNodes(TreeNode *root1_current, TreeNode *root2_current)
    {
        if (root2_current == nullptr)
            return;

        root1_current->val += root2_current->val;

        if (root2_current->left != nullptr)
        {
            if (root1_current->left == nullptr)
            {
                root1_current->left = root2_current->left;
            }
            else
            {
                sumNodes(root1_current->left, root2_current->left);
            }
        }

        if (root2_current->right != nullptr)
        {
            if (root1_current->right == nullptr)
            {
                root1_current->right = root2_current->right;
            }
            else
            {
                sumNodes(root1_current->right, root2_current->right);
            }
        }
    }

public:
    TreeNode *mergeTrees(TreeNode *root1, TreeNode *root2)
    {
        if (root1 == nullptr)
        {
            return root2;
        }
        sumNodes(root1, root2);
        return root1;
    }
};