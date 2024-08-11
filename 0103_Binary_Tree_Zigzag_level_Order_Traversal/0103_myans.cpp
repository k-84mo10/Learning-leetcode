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
    vector<vector<int>> zigzagLevelOrder(TreeNode *root)
    {
        vector<vector<int>> zigzag_level_order_arr;

        if (root == nullptr)
            return zigzag_level_order_arr;

        queue<TreeNode *> current_detected_node_queue;
        current_detected_node_queue.push(root);
        int level = 0;

        while (!current_detected_node_queue.empty())
        {
            int qsize = current_detected_node_queue.size();
            vector<int> current_level_arr;
            for (int i = 0; i < qsize; i++)
            {
                TreeNode *node = current_detected_node_queue.front();
                current_detected_node_queue.pop();

                if (node->left != nullptr)
                    current_detected_node_queue.push(node->left);
                if (node->right != nullptr)
                    current_detected_node_queue.push(node->right);

                current_level_arr.push_back(node->val);
            }
            if (level % 2 == 1)
                reverse(current_level_arr.begin(), current_level_arr.end());
            zigzag_level_order_arr.push_back(current_level_arr);
            level++;
        }

        return zigzag_level_order_arr;
    }
};