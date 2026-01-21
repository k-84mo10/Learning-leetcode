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
    vector<string> binaryTreePaths(TreeNode* root) {
        vector<string> paths;
        string path;
        dfs(root, path, paths);
        return paths;
    }

    void dfs(TreeNode* node, string& path, vector<string>& paths) {
        const size_t old = path.size();

        if (!path.empty()) path += "->";
        path += std::to_string(node->val);

        if (node->left == nullptr & node->right == nullptr) {
            paths.push_back(path);
        } else {
            if (node->left != nullptr) dfs(node->left, path, paths);
            if (node->right != nullptr) dfs(node->right, path, paths);
        }

        path.resize(old);
    }
};