class Solution {
public:
    string gcdOfStrings(string str1, string str2) {
        int i = 0;
        while (i < str1.size() && i < str2.size() && str1[i] == str2[i]) i++;
        if (i == 0) return "";
        if (i == str1.size() && i == str2.size()) return str1;

        while (i > 0) {
            if (str1.size() % i != 0 || str2.size() % i != 0) {
                i--;
                continue;
            }

            for (int j = i; j < max(str1.size(), str2.size()); j++) {
                if (j < str1.size() && str1[j] != str1[j%i]) break;
                if (j < str2.size() && str2[j] != str2[j%i]) break;
                if (j == max(str1.size()-1, str2.size()-1)) return str1.substr(0, i);
            } 

            i--;
        }

        return "";
    }
};