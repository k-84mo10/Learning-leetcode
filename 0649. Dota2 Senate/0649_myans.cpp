class Solution
{
public:
    string predictPartyVictory(string senate)
    {
        queue<int> radiant;
        queue<int> dire;

        for (int i = 0; i < senate.size(); i++)
        {
            if (senate[i] == 'R')
            {
                radiant.push(i);
            }
            else
            {
                dire.push(i);
            }
        }

        while (true)
        {
            if (radiant.empty())
                return "Dire";
            if (dire.empty())
                return "Radiant";

            int rad_front = radiant.front();
            int dir_front = dire.front();

            radiant.pop();
            dire.pop();

            if (rad_front < dir_front)
            {
                radiant.push(rad_front + senate.size());
            }
            else
            {
                dire.push(dir_front + senate.size());
            }
        }
    }
};