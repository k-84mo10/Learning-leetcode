class Solution {
public:
    string gcdOfStrings(string str1, string str2) {
        int inter_num = 0;
        while (inter_num < str1.size() && inter_num < str2.size() && str1[inter_num] == str2[inter_num]) inter_num++;
        if (inter_num == 0) return "";
        if (inter_num == str1.size() && inter_num == str2.size()) return str1;

        while (inter_num > 0) {
            if (str1.size() % inter_num != 0 || str2.size() % inter_num != 0) {
                inter_num--;
                continue;
            }

            for (int j = inter_num; j < max(str1.size(), str2.size()); j++) {
                if (j < str1.size() && str1[j] != str1[j%inter_num]) break;
                if (j < str2.size() && str2[j] != str2[j%inter_num]) break;
                if (j == max(str1.size()-1, str2.size()-1)) return str1.substr(0, inter_num);
            } 

            inter_num--;
        }

        return "";
    }
};