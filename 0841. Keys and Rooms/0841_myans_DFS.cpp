class Solution
{
public:
    bool canVisitAllRooms(vector<vector<int>> &rooms)
    {
        int n = rooms.size();
        vector<bool> can_enter_room(n, false);
        stack<int> s;

        s.push(0);

        while (!s.empty())
        {
            int room_num = s.top();
            s.pop();

            if (!can_enter_room[room_num])
            {
                can_enter_room[room_num] = true;
                for (int &room : rooms[room_num])
                {
                    s.push(room);
                }
            }
        }

        for (int i = 0; i < n; i++)
        {
            if (!can_enter_room[i])
                return false;
        }

        return true;
    }
};