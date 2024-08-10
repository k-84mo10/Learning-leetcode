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
    void inputVal(TreeNode *root, int level, vector<vector<int>> &order)
    {
        if (root == nullptr)
            return;

        if (order.size() < level + 1)
        {
            order.push_back(vector<int>{root->val});
        }
        else
        {
            order[level].push_back(root->val);
        }

        inputVal(root->left, level + 1, order);
        inputVal(root->right, level + 1, order);
    }

public:
    vector<vector<int>> levelOrder(TreeNode *root)
    {
        vector<vector<int>> answer;
        inputVal(root, 0, answer);
        return answer;
    }
};