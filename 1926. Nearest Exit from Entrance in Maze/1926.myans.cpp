class Solution
{
public:
    int nearestExit(vector<vector<char>> &maze, vector<int> &entrance)
    {
        int row_size = maze.size();
        int col_size = maze[0].size();
        vector<vector<int>> detected(row_size, vector<int>(col_size, -1));
        queue<vector<int>> route;

        detected[entrance[0]][entrance[1]] = 0;
        route.push(entrance);

        while (!route.empty())
        {
            vector<int> cell = route.front();
            route.pop();
            int count = detected[cell[0]][cell[1]];

            if ((cell[0] == 0 || cell[0] == row_size - 1 || cell[1] == 0 || cell[1] == col_size - 1) && count != 0)
            {
                return count;
            }

            if (cell[0] > 0 && maze[cell[0] - 1][cell[1]] == '.' && detected[cell[0] - 1][cell[1]] == -1)
            {
                route.push({cell[0] - 1, cell[1]});
                detected[cell[0] - 1][cell[1]] = count + 1;
            }
            if (cell[0] < row_size - 1 && maze[cell[0] + 1][cell[1]] == '.' && detected[cell[0] + 1][cell[1]] == -1)
            {
                route.push({cell[0] + 1, cell[1]});
                detected[cell[0] + 1][cell[1]] = count + 1;
            }
            if (cell[1] > 0 && maze[cell[0]][cell[1] - 1] == '.' && detected[cell[0]][cell[1] - 1] == -1)
            {
                route.push({cell[0], cell[1] - 1});
                detected[cell[0]][cell[1] - 1] = count + 1;
            }
            if (cell[1] < col_size - 1 && maze[cell[0]][cell[1] + 1] == '.' && detected[cell[0]][cell[1] + 1] == -1)
            {
                route.push({cell[0], cell[1] + 1});
                detected[cell[0]][cell[1] + 1] = count + 1;
            }
        }

        return -1;
    }
};