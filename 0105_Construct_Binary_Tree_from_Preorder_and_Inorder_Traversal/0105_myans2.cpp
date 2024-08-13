class Solution
{
    TreeNode *buildTreeHelper(int leftindex, int rightindex, int &preorderindex, vector<int> &preorder, unordered_map<int, int> &inordermap)
    {
        if (leftindex > rightindex)
            return nullptr;

        int rootVal = preorder[preorderindex++];
        TreeNode *root = new TreeNode(rootVal);

        int inorderindex = inordermap[rootVal];

        root->left = buildTreeHelper(leftindex, inorderindex - 1, preorderindex, preorder, inordermap);
        root->right = buildTreeHelper(inorderindex + 1, rightindex, preorderindex, preorder, inordermap);

        return root;
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

        int preorderindex = 0;
        return buildTreeHelper(0, nums_size - 1, preorderindex, preorder, inordermap);
    }
};
