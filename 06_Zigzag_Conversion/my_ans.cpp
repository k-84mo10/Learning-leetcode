class Solution {
public:
    string convert(string s, int numRows) {
        vector<string> string_list(numRows);
        if (numRows == 1) return s;

        for (int i = 0; i < s.size(); i++) {
            int remainder = i % (2*numRows-2);
            if (remainder == 0) {
                string_list[0] += s[i];
            }
            else if (remainder == numRows-1) {
                string_list[numRows-1] += s[i];
            }
            else {
                int locate = (remainder-numRows+1 > 0) ? remainder-numRows+1 : (remainder-numRows+1)*(-1);
                string_list[numRows-1-locate] += s[i];
            }
        }

        string result;
        for (string& row: string_list) {
            result += row;
        }
        return result;
    }
};