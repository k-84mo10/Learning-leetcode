class Solution
{
    int countIslandArea(vector<vector<int>> &grid, int i, int j, int &area)
    {
        if (grid[i][j] == 1)
        {
            area++;
            grid[i][j] = 2;
            if (i != 0)
                countIslandArea(grid, i - 1, j, area);
            if (i != grid.size() - 1)
                countIslandArea(grid, i + 1, j, area);
            if (j != 0)
                countIslandArea(grid, i, j - 1, area);
            if (j != grid[0].size() - 1)
                countIslandArea(grid, i, j + 1, area);
        }

        return area;
    }

public:
    int maxAreaOfIsland(vector<vector<int>> &grid)
    {
        int width = grid.size();
        int height = grid[0].size();
        int maxIslandArea = 0;

        for (int i = 0; i < width; i++)
        {
            for (int j = 0; j < height; j++)
            {
                if (grid[i][j] == 1)
                {
                    int area = 0;
                    int currentArea = countIslandArea(grid, i, j, area);
                    if (currentArea > maxIslandArea)
                        maxIslandArea = currentArea;
                }
            }
        }
        return maxIslandArea;
    }
};