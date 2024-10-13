class Solution
{
public:
    vector<int> asteroidCollision(vector<int> &asteroids)
    {
        vector<int> left_asteroid;
        for (int &asteroid : asteroids)
        {
            if (asteroid < 0 && !left_asteroid.empty() && left_asteroid.back() > 0)
            {
                while (left_asteroid.back() > 0 && left_asteroid.back() <= asteroid * (-1))
                {
                    if (left_asteroid.back() + asteroid == 0)
                    {
                        left_asteroid.pop_back();
                        break;
                    }
                    left_asteroid.pop_back();
                    if (left_asteroid.empty() || left_asteroid.back() < 0)
                        left_asteroid.push_back(asteroid);
                }
            }
            else
            {
                left_asteroid.push_back(asteroid);
            }
        }
        return left_asteroid;
    }
};