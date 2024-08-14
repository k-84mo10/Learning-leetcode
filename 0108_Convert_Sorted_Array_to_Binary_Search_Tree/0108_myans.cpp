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
    TreeNode *createNode(vector<int> &nums, int left, int right)
    {
        if (left > right)
            return nullptr;

        int medium = left + (right - left) / 2;

        TreeNode *current = new TreeNode(nums[medium]);

        current->left = createNode(nums, left, medium - 1);
        current->right = createNode(nums, medium + 1, right);

        return current;
    }

public:
    TreeNode *sortedArrayToBST(vector<int> &nums)
    {
        return createNode(nums, 0, nums.size() - 1);
    }
};