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
class Solution
{
    void createChildNode(TreeNode *currentNode, int leftindex, int rightindex, int preorderindex, vector<int> &preorder, unordered_map<int, int> inordermap)
    {
        int nums_size = preorder.size();
        int currentindex = inordermap[currentNode->val];
        int j = preorderindex + 1;
        if (leftindex < currentindex)
        {
            while (j < nums_size)
            {
                int inorderindex = inordermap[preorder[j]];
                if (leftindex <= inorderindex && inorderindex < currentindex)
                {
                    currentNode->left = new TreeNode(preorder[j]);
                    createChildNode(currentNode->left, leftindex, currentindex - 1, j, preorder, inordermap);
                    break;
                }
                j++;
            }
        }
        else
        {
            currentNode->left = nullptr;
        }

        if (currentindex < rightindex)
        {
            while (j < nums_size)
            {
                int inorderindex = inordermap[preorder[j]];
                if (currentindex < inorderindex && inorderindex <= rightindex)
                {
                    currentNode->right = new TreeNode(preorder[j]);
                    createChildNode(currentNode->right, currentindex + 1, rightindex, j, preorder, inordermap);
                    break;
                }
                j++;
            }
        }
        else
        {
            currentNode->right = nullptr;
        }
    }

public:
    TreeNode *buildTree(vector<int> &preorder, vector<int> &inorder)
    {
        unordered_map<int, int> inordermap;
        int nums_size = inorder.size();

        for (int i = 0; i < nums_size; i++)
        {
            inordermap[inorder[i]] = i;
        }

        TreeNode *root = new TreeNode(preorder[0]);
        createChildNode(root, 0, nums_size - 1, 0, preorder, inordermap);

        return root;
    }
};