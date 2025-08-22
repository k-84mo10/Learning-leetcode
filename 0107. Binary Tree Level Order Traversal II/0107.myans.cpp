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
    vector<vector<int>> levelOrderBottom(TreeNode* root) {
        std::queue<TreeNode*> q;
        std::vector<vector<int>> answer;

        if (!root) return answer;

        q.push(root);

        while (!q.empty()) {
            int size = q.size();
            std::vector<int> current;
            for (int i=0; i<size; i++) {
                TreeNode* node = q.front();
                q.pop();
                current.push_back(node->val);
                if (node->left) q.push(node->left);
                if (node->right) q.push(node->right);
            }
            answer.push_back(current);
        }
        std::reverse(answer.begin(), answer.end());
        return answer;
    }
};