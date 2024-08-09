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
    bool isValInRange(TreeNode *parent, int minimum, int maximum)
    {
        if (minimum > parent->val || parent->val > maximum)
            return false;

        if (parent->left != nullptr)
        {
            if (parent->val == numeric_limits<int>::min())
                return false;
            if (isValInRange(parent->left, minimum, parent->val - 1) == false)
                return false;
        }

        if (parent->right != nullptr)
        {
            if (parent->val == numeric_limits<int>::max())
                return false;
            if (isValInRange(parent->right, parent->val + 1, maximum) == false)
                return false;
        }

        return true;
    }

public:
    bool isValidBST(TreeNode *root)
    {
        return isValInRange(root, numeric_limits<int>::min(), numeric_limits<int>::max());
    }
};