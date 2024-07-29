class Solution
{
public:
    int uniquePathsWithObstacles(vector<vector<int>> &obstacleGrid)
    {
        int width = obstacleGrid[0].size();
        int height = obstacleGrid.size();
        vector<vector<int>> pathGrid(height, vector<int>(width, 0));
        pathGrid[0][0] = 1;

        for (int i = 0; i < height; i++)
        {
            for (int j = 0; j < width; j++)
            {
                if (obstacleGrid[i][j] == 1)
                {
                    pathGrid[i][j] = 0;
                }
                else
                {
                    if (i > 0)
                    {
                        pathGrid[i][j] += pathGrid[i - 1][j];
                    }
                    if (j > 0)
                    {
                        pathGrid[i][j] += pathGrid[i][j - 1];
                    }
                }
            }
        }

        return pathGrid[height - 1][width - 1];
    }
};