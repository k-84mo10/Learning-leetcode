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
    vector<vector<int>> ps; 

public:
    Node* construct(vector<vector<int>>& grid) {
        buildPrefix(grid);
        return construct(grid, 0, 0, grid.size());
    }

    void buildPrefix(vector<vector<int>>& grid) {
        int n = (int)grid.size();
        ps.assign(n+1, vector<int>(n+1, 0));
        
        for (int row = 0; row < n; row++) {
            for (int col = 0; col < n; col++) {
                ps[row+1][col+1] = ps[row+1][col] + ps[row][col+1] - ps[row][col] + grid[row][col];
            }
        }
    }

    int rangeSum(int row, int col, int len) {
        int r1 = row;
        int c1 = col;
        int r2 = row + len;
        int c2 = col + len;
        return ps[r2][c2] - ps[r2][c1] - ps[r1][c2] + ps[r1][c1];
    }

    Node* construct(vector<vector<int>>& grid, int row, int col, int len) {
        int sum = rangeSum(row, col, len);

        // すべてが 1 あるいは 0 なら isLeaf=1 にしてreturn
        if (sum == 0) return new Node(false, true);
        if (sum == len*len) return new Node(true, true);

        Node* node = new Node(false, false);
        int half = len/2;
        node->topLeft = construct(grid, row, col, half);
        node->topRight = construct(grid, row, col+half, half);
        node->bottomLeft = construct(grid, row+half, col, half);
        node->bottomRight = construct(grid, row+half, col+half, half);

        return node;
    }
};