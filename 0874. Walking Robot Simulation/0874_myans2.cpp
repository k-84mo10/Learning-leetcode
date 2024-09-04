#include <vector>
#include <set>
#include <utility>   // for pair
#include <algorithm> // for max
using namespace std;

class Solution
{
    class Robots
    {
        array<int, 2> robots_position = {0, 0};
        array<int, 2> robots_vector = {0, 1};
        bool is_stack = false;

    public:
        explicit Robots() {};
        ~Robots() {};

        void turn_left()
        {
            int next_vector_x = -robots_vector[1];
            int next_vector_y = robots_vector[0];
            robots_vector[0] = next_vector_x;
            robots_vector[1] = next_vector_y;
        }

        void turn_right()
        {
            int next_vector_x = robots_vector[1];
            int next_vector_y = -robots_vector[0];
            robots_vector[0] = next_vector_x;
            robots_vector[1] = next_vector_y;
        }

        void walk(set<pair<int, int>> &obstacles_set)
        {
            is_stack = false;
            pair<int, int> next_position = {robots_position[0] + robots_vector[0], robots_position[1] + robots_vector[1]};
            if (obstacles_set.find(next_position) == obstacles_set.end())
            {
                robots_position[0] = next_position.first;
                robots_position[1] = next_position.second;
            }
            else
            {
                is_stack = true;
            }
        }

        int get_square() const
        {
            return robots_position[0] * robots_position[0] + robots_position[1] * robots_position[1];
        }

        bool get_stack() const
        {
            return is_stack;
        }
    };

public:
    int robotSim(vector<int> &commands, vector<vector<int>> &obstacles)
    {
        set<pair<int, int>> obstacles_set;
        for (const vector<int> &obstacle : obstacles)
        {
            obstacles_set.insert({obstacle[0], obstacle[1]});
        }

        Robots robots;
        int max_square = 0;
        for (int i = 0; i < commands.size(); i++)
        {
            if (commands[i] == -2)
            {
                robots.turn_left();
            }
            else if (commands[i] == -1)
            {
                robots.turn_right();
            }
            else
            {
                for (int j = 0; j < commands[i]; j++)
                {
                    robots.walk(obstacles_set);
                    if (robots.get_stack())
                        break;
                    max_square = max(max_square, robots.get_square());
                }
            }
        }
        return max_square;
    }
};
