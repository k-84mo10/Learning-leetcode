class Solution
{
    int countIslandArea(vector<vector<int>> &grid, int i, int j)
    {
        grid[i][j] = 2;
        queue<vector<int>> q;
        int area = 1;
        if (i != 0 && grid[i - 1][j] == 1)
            q.push(vector<int>{i - 1, j});
        if (i != grid.size() - 1 && grid[i + 1][j] == 1)
            q.push(vector<int>{i + 1, j});
        if (j != 0 && grid[i][j - 1] == 1)
            q.push(vector<int>{i, j - 1});
        if (j != grid[0].size() - 1 && grid[i][j + 1] == 1)
            q.push(vector<int>{i, j + 1});

        while (!q.empty())
        {
            vector<int> v = q.front();
            q.pop();
            int current_i = v[0];
            int current_j = v[1];
            if (grid[current_i][current_j] != 1)
                continue;
            grid[current_i][current_j] = 2;
            area += 1;
            if (current_i != 0 && grid[current_i - 1][current_j] == 1)
                q.push(vector<int>{current_i - 1, current_j});
            if (current_i != grid.size() - 1 && grid[current_i + 1][current_j] == 1)
                q.push(vector<int>{current_i + 1, current_j});
            if (current_j != 0 && grid[current_i][current_j - 1] == 1)
                q.push(vector<int>{current_i, current_j - 1});
            if (current_j != grid[0].size() - 1 && grid[current_i][current_j + 1] == 1)
                q.push(vector<int>{current_i, current_j + 1});
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
                    int currentArea = countIslandArea(grid, i, j);
                    if (currentArea > maxIslandArea)
                        maxIslandArea = currentArea;
                }
            }
        }
        return maxIslandArea;
    }
};