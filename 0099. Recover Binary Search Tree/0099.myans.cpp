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
    void recoverTree(TreeNode* root) {
        TreeNode* current = root;
        TreeNode* previous = nullptr;
        TreeNode* first = nullptr;
        TreeNode* second = nullptr;

        while (current) {
            if (!current->left) {
                if (previous && previous->val > current->val) {
                    if (!first) first = previous;
                    second = current;
                }
                previous = current;
                current = current->right;
            } else {
                TreeNode* thread = current->left;
                while (thread->right && thread->right != current) {
                    thread = thread->right;
                }
                if (!thread->right) {
                    thread->right = current;
                    current = current->left;
                } else {
                    thread->right = nullptr;
                    if (previous && previous->val > current->val) {
                        if (!first) first = previous;
                        second = current;
                    }
                    previous = current;
                    current = current->right;
                }
            }
        }

        std::swap(first->val, second->val);
    }
};