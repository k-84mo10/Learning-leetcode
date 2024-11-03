class Solution
{
public:
    int nearestExit(vector<vector<char>> &maze, vector<int> &entrance)
    {
        int row_size = maze.size();
        int col_size = maze[0].size();
        queue<tuple<int, int, int>> route;
        route.push({entrance[0], entrance[1], 0});

        while (!route.empty())
        {
            int cell_row = get<0>(route.front());
            int cell_col = get<1>(route.front());
            int count = get<2>(route.front());
            route.pop();

            if (cell_row < 0 || cell_row >= row_size || cell_col < 0 || cell_col >= col_size)
                continue;
            if (maze[cell_row][cell_col] == '+')
                continue;

            if ((cell_row == 0 || cell_row == row_size - 1 || cell_col == 0 || cell_col == col_size - 1) && count != 0)
                return count;
            maze[cell_row][cell_col] = '+';

            route.push(make_tuple(cell_row - 1, cell_col, count + 1));
            route.push(make_tuple(cell_row + 1, cell_col, count + 1));
            route.push(make_tuple(cell_row, cell_col - 1, count + 1));
            route.push(make_tuple(cell_row, cell_col + 1, count + 1));
        }

        return -1;
    }
};