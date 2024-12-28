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
    bool leafSimilar(TreeNode* root1, TreeNode* root2) {
        std::vector<int> root1LeafValue = leafarray(root1);
        std::vector<int> root2LeafValue = leafarray(root2);

        if (root1LeafValue.size() != root2LeafValue.size()) return false;

        for (int i = 0; i < root1LeafValue.size(); i++) {
            if (root1LeafValue[i] != root2LeafValue[i]) return false;
        }

        return true;
    }

    std::vector<int> leafarray(TreeNode* root1) {
        std::stack<TreeNode*> nodeStack;
        std::vector<int> leafValue;
        if (root1 == nullptr) return leafValue;

        nodeStack.push(root1);

        while (!nodeStack.empty()) {
            TreeNode* node = nodeStack.top();
            nodeStack.pop();

            if (node->left == nullptr && node->right == nullptr) {
                leafValue.push_back(node->val);
            } else {
                if (node->left != nullptr) nodeStack.push(node->left);
                if (node->right != nullptr) nodeStack.push(node->right);
            }
        }

        return leafValue;
    }
};