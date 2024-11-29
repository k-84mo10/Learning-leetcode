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

        int intMin = std::numeric_limits<int>::min();
        int intMax = std::numeric_limits<int>::max();

        std::stack<std::tuple<TreeNode*, int, std::unordered_map<int, int>>> s;
        std::unordered_map<int, int> sumCountMap;
        sumCountMap[0] = 1;

        s.push(std::make_tuple(root, 0, sumCountMap));
        int count = 0;

        while (!s.empty()) {
            auto [currentNode, currentSum, sumCountMap] = s.top();
            s.pop();

            if (currentNode == nullptr) continue;

            if (currentNode->val > 0 && currentSum > intMax - currentNode->val) continue;
            if (currentNode->val < 0 && currentSum < intMin - currentNode->val) continue;

            currentSum += currentNode->val;
            count += sumCountMap[currentSum-targetSum];
            sumCountMap[currentSum] ++;

            s.push(std::make_tuple(currentNode->left, currentSum, sumCountMap));
            s.push(std::make_tuple(currentNode->right, currentSum, sumCountMap));
        }

        return count;
    }
};