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
public:
    vector<vector<int>> levelOrder(TreeNode *root)
    {
        vector<vector<int>> order;
        queue<TreeNode *> q;

        if (root == nullptr)
            return order;
        q.push(root);

        while (!q.empty())
        {
            int qsize = q.size();
            vector<int> arr;
            for (int i = 0; i < qsize; i++)
            {
                TreeNode *node = q.front();
                q.pop();
                if (node->left != nullptr)
                    q.push(node->left);
                if (node->right != nullptr)
                    q.push(node->right);
                arr.push_back(node->val);
            }
            order.push_back(arr);
        }

        return order;
    }
};