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
    vector<TreeNode*> generateTrees(int n) {
        return buildTrees(1, n);
    }

    vector<TreeNode*> buildTrees(int low, int high) {
        if (low > high) return {nullptr};
        vector<TreeNode*> answer;

        for (int i = low; i <= high; i++) {
            vector<TreeNode*> leftTrees = buildTrees(low, i-1);
            vector<TreeNode*> rightTrees = buildTrees(i+1, high);

            for (TreeNode* left : leftTrees) {
                for (TreeNode* right : rightTrees) {
                    answer.push_back(new TreeNode(i, left, right));
                }
            }
        }

        return answer;
    }
};