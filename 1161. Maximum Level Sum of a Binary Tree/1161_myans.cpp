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
    int maxLevelSum(TreeNode* root) {
        std::queue<TreeNode*> q;
        int maximum_level = 0;
        int maximum_sum = -1e5-1;

        if (root == nullptr) return maximum_level;
        q.push(root);
        
        int level = 0;
        while (!q.empty()) {
            int size = q.size();
            int level_sum = 0;
            level++;

            for (int i = 0; i < size; i++) {
                TreeNode* node = q.front();
                q.pop();

                level_sum += node->val;
                if (node->left != nullptr) q.push(node->left);
                if (node->right != nullptr) q.push(node->right);
            }

            if (maximum_sum < level_sum) {
                maximum_level = level;
                maximum_sum = level_sum;
            }
        }

        return maximum_level;
    }
};