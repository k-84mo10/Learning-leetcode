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
        int kthSmallest(TreeNode* root, int k) {
            std::priority_queue<int> pq;
            std::queue<TreeNode*> q; 
    
            q.push(root);
    
            while (!q.empty()) {
                auto node = q.front();
                q.pop();
    
                pq.push(node->val);
                if (pq.size() > k) pq.pop();
    
                if (node->left != nullptr) q.push(node->left);
                if (node->right != nullptr) q.push(node->right);
            }
            return pq.top();
        }
    };