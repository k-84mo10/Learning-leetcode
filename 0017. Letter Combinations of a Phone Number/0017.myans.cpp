class Solution {
    std::vector<std::vector<char>> map_of_phone = {
        {},
        {},
        {'a', 'b', 'c'},
        {'d', 'e', 'f'},
        {'g', 'h', 'i'},
        {'j', 'k', 'l'},
        {'m', 'n', 'o'},
        {'p', 'q', 'r', 's'},
        {'t', 'u', 'v'},
        {'w', 'x', 'y', 'z'}
    };

public:
    vector<string> letterCombinations(string digits) {
        std::vector<string> result;
        if (digits == "") return result;

        std::queue<string> q;
        q.push("");

        for (int i = 0; i < digits.size(); i++) {
            int num = digits[i] - '0';
            int size = q.size();
            for (int j = 0; j < size; j++) {
                string str = q.front();
                q.pop();
                for (char c : map_of_phone[num]) {
                    q.push(str+c);
                }
            }
        }
        while (!q.empty()) {
            result.push_back(q.front());
            q.pop();
        }

        return result;
    }
};