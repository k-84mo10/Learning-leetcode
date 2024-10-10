class Solution
{
public:
    int equalPairs(vector<vector<int>> &grid)
    {
        map<vector<int>, int> vecmap;
        for (vector<int> &vec : grid)
        {
            vecmap[vec]++;
        }

        int count = 0;

        for (int i = 0; i < grid.size(); i++)
        {
            vector<int> vec(grid.size(), 0);
            for (int j = 0; j < grid.size(); j++)
            {
                vec[j] = grid[j][i];
            }
            if (vecmap.find(vec) != vecmap.end())
            {
                count += vecmap[vec];
            }
        }

        return count;
    }
};