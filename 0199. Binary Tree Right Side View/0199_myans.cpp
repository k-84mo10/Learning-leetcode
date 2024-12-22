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
    vector<int> rightSideView(TreeNode* root) {
        std::queue<std::pair<TreeNode*, int>> q;
        std::vector<int> result;

        if (root != nullptr) {
            q.push(std::make_pair(root, 0));
        }

        while (!q.empty()) {
            auto [currentNode, level] = q.front();
            q.pop();

            if (result.size() <= level) {
                result.push_back(currentNode->val);
            } else {
                result[level] = currentNode->val;
            }

            if (currentNode->left != nullptr) q.push(std::make_pair(currentNode->left, level+1));
            if (currentNode->right != nullptr) q.push(std::make_pair(currentNode->right, level+1));
        }

        return result;
    }
};