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
    int pathSum(TreeNode* root, int targetSum) {
        if (root == nullptr) return 0;
        int count = 0;

        int intMin = std::numeric_limits<int>::min();
        int intMax = std::numeric_limits<int>::max();

        std::stack<std::tuple<TreeNode*, int, bool>> s;
        s.push(std::make_tuple(root, targetSum, true)); 

        while (!s.empty()) {
            auto current = s.top();
            s.pop();
            TreeNode* currentNode = std::get<0>(current);
            int currentTargetSum = std::get<1>(current);
            bool is_start = std::get<2>(current);

            if (currentNode == nullptr) continue;
            if (currentNode->val == currentTargetSum) count++;

            if (currentNode->val > 0 && currentNode->val + intMin > currentTargetSum) continue;
            if (currentNode->val < 0 && currentTargetSum > intMax + currentNode->val) continue;
            
            int nextTargetSum = currentTargetSum - currentNode->val;
            if (currentNode->left != nullptr) {
                s.push(std::make_tuple(currentNode->left, nextTargetSum, false));
                if (is_start) s.push(std::make_tuple(currentNode->left, targetSum, true));
            }
            if (currentNode->right != nullptr) {
                s.push(std::make_tuple(currentNode->right, nextTargetSum, false));
                if (is_start) s.push(std::make_tuple(currentNode->right, targetSum, true));
            }
        }

        return count;
    }
};