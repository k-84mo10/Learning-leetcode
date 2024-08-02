class Solution
{
    void detectIslandSize(vector<vector<char>> &grid, int i, int j)
    {
        if (grid[i][j] == '1')
        {
            grid[i][j] = '2';
            if (i != 0)
                detectIslandSize(grid, i - 1, j);
            if (j != 0)
                detectIslandSize(grid, i, j - 1);
            if (i != grid.size() - 1)
                detectIslandSize(grid, i + 1, j);
            if (j != grid[0].size() - 1)
                detectIslandSize(grid, i, j + 1);
        }
    }

public:
    int numIslands(vector<vector<char>> &grid)
    {
        int height_size = grid.size();
        int width_size = grid[0].size();
        int count = 0;

        for (int i = 0; i < height_size; i++)
        {
            for (int j = 0; j < width_size; j++)
            {
                if (grid[i][j] == '1')
                {
                    detectIslandSize(grid, i, j);
                    count++;
                }
            }
        }

        return count;
    }
};