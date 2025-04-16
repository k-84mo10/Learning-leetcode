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
        TreeNode* deleteNode(TreeNode* root, int key) {
            if (root == nullptr) return nullptr;
    
            if (root->val > key) {
                root->left = deleteNode(root->left, key);
                return root;
            } 
            if (root->val < key) {
                root->right = deleteNode(root->right, key);
                return root;
            }
    
            if (root->left == nullptr) {
                return root->right;
            }
            if (root->right == nullptr) {
                return root->left;
            }
    
            TreeNode* minNode = findMinNode(root->right);
            root->val = minNode->val;
            root->right = deleteNode(root->right, root->val);
    
            return root;
        }
    
        TreeNode* findMinNode(TreeNode* node) {
            while (node->left != nullptr) node = node->left;
            return node;
        }
    };