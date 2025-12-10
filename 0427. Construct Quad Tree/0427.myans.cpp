/*
// Definition for a QuadTree node.
class Node {
public:
    bool val;
    bool isLeaf;
    Node* topLeft;
    Node* topRight;
    Node* bottomLeft;
    Node* bottomRight;
    
    Node() {
        val = false;
        isLeaf = false;
        topLeft = NULL;
        topRight = NULL;
        bottomLeft = NULL;
        bottomRight = NULL;
    }
    
    Node(bool _val, bool _isLeaf) {
        val = _val;
        isLeaf = _isLeaf;
        topLeft = NULL;
        topRight = NULL;
        bottomLeft = NULL;
        bottomRight = NULL;
    }
    
    Node(bool _val, bool _isLeaf, Node* _topLeft, Node* _topRight, Node* _bottomLeft, Node* _bottomRight) {
        val = _val;
        isLeaf = _isLeaf;
        topLeft = _topLeft;
        topRight = _topRight;
        bottomLeft = _bottomLeft;
        bottomRight = _bottomRight;
    }
};
*/

class Solution {
public:
    Node* construct(vector<vector<int>>& grid) {
        return construct(grid, 0, 0, grid.size());
    }

    Node* construct(vector<vector<int>>& grid, int col, int row, int n) {
        // 範囲内の合計を計算
        int sum = 0;
        for (int i = col; i < col + n; i++) {
            for (int j = row; j < row + n; j++) {
                sum += grid[i][j];
            }
        }

        // すべてが 1 あるいは 0 なら isLeaf=1 にしてreturn
        if (sum == 0) return new Node(false, true);
        if (sum == n*n) return new Node(true, true);

        Node* node = new Node(false, false);
        node->topLeft = construct(grid, col, row, n/2);
        node->topRight = construct(grid, col, row+n/2, n/2);
        node->bottomLeft = construct(grid, col+n/2, row, n/2);
        node->bottomRight = construct(grid, col+n/2, row+n/2, n/2);

        return node;
    }
};