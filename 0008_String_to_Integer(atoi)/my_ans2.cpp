class Solution {
public:
    int myAtoi(string s) {
        bool flag_first_digit = 0;
        bool flag_minus = 0;
        bool flag_plus = 0;

        int answer = 0;

        for (int i = 0; i < s.size(); i++) {
            int string_code = s[i] - '0';
            if (s[i] == ' ') {
                if (flag_first_digit || flag_minus || flag_plus) break; 
            } else if (s[i] == '+') {
                if (flag_first_digit || flag_minus || flag_plus) break; 
                flag_plus = 1;
            } else if (s[i] == '-') {
                if (flag_first_digit || flag_minus || flag_plus) break; 
                flag_minus = 1;
            } else if (string_code >= 0 && string_code < 10) {
                flag_first_digit = 1;
                if (answer > (numeric_limits<int>::max()/10)) {
                    return flag_minus ? numeric_limits<int>::min() : numeric_limits<int>::max();
                } else {
                    answer *= 10;
                }
                if (answer > (numeric_limits<int>::max()-s[i]+'0')) {
                    return flag_minus ? numeric_limits<int>::min() : numeric_limits<int>::max() ;
                } else {
                    answer += string_code;
                }
            } else {
                break;
            }
        }

        answer = flag_minus ? answer*(-1) : answer;
        return answer;
    }
};