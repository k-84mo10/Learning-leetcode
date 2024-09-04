class Solution
{
public:
    int robotSim(vector<int> &commands, vector<vector<int>> &obstacles)
    {
        set<vector<int>> obstacles_set;
        for (vector<int> &obstacle : obstacles)
        {
            obstacles_set.insert(obstacle);
        }

        int x = 0;
        int y = 0;
        int max_squared = x * x + y * y;
        vector<int> robots_vector = {0, 1};

        for (int i = 0; i < commands.size(); i++)
        {
            if (commands[i] == -2)
            {
                vector<int> next_robots_vector = robots_vector;
                next_robots_vector[0] = -1 * robots_vector[1];
                next_robots_vector[1] = robots_vector[0];
                robots_vector = next_robots_vector;
            }
            else if (commands[i] == -1)
            {
                vector<int> next_robots_vector = robots_vector;
                next_robots_vector[0] = robots_vector[1];
                next_robots_vector[1] = -1 * robots_vector[0];
                robots_vector = next_robots_vector;
            }
            else
            {
                int next_x = x;
                int next_y = y;
                for (int j = 0; j < commands[i]; j++)
                {
                    next_x = x + 1 * robots_vector[0];
                    next_y = y + 1 * robots_vector[1];
                    vector<int> next_position = {next_x, next_y};
                    if (obstacles_set.find(next_position) != obstacles_set.end())
                        break;
                    x = next_x;
                    y = next_y;
                    max_squared = max(x * x + y * y, max_squared);
                }
            }
        }

        return max_squared;
    }
};