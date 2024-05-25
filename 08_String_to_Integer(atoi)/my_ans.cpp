class Solution {
public:
    int myAtoi(string s) {
        int answer = 0;
        int flag_first_digit = 0;
        int flag_minus = 0;
        int flag_plus = 0;
        for (int i=0; i<s.size(); i++) {
            int string_code = s[i] - '0';
            if (string_code >= 0 && string_code < 10) {
                if (flag_first_digit == 0) flag_first_digit = 1;
                if (answer >  (numeric_limits<int>::max() / 10)) {
                    if (flag_minus == 1) return numeric_limits<int>::min();
                    return numeric_limits<int>::max();
                } 
                answer *= 10;
                if (answer >  (numeric_limits<int>::max() - string_code)) {
                    if (flag_minus == 1) return numeric_limits<int>::min();
                    return numeric_limits<int>::max();
                } 
                answer += string_code;
            } else if (string_code == -3) {
                if (flag_minus == 1 || flag_plus == 1 || flag_first_digit == 1) break;
                flag_minus = 1;
            } else if (string_code == -16) {
                if (flag_minus == 1 || flag_plus == 1 || flag_first_digit == 1) break;
            } else if (string_code == -5) {
                if (flag_minus == 1 || flag_plus == 1 || flag_first_digit == 1) break;
                flag_plus = 1;
            } else {
                break;
            }
        }
        if (flag_minus == 1) answer *= -1;
        return answer;
    }
};