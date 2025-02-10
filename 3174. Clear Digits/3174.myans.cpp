class Solution {
public:
    string clearDigits(string s) {
        int num_digit = 0;

        const int s_size = s.size();
        std::string answer;

        for (int i = s_size - 1; i >= 0; i--) {
            if (s[i] - '0' >= 0 && s[i] - '0' < 10) {
                num_digit++;
            } else {
                if (num_digit > 0) {
                    num_digit--;
                } else {
                    answer += s[i];
                }
            }
        }

        std::reverse(answer.begin(), answer.end());
        return answer;
    }
};
