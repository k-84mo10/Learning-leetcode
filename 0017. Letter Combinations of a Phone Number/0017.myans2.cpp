class Solution {
    std::vector<string> map_of_phone = {
        "",
        "",
        "abc",
        "def",
        "ghi",
        "jkl",
        "mno",
        "pqrs",
        "tuv",
        "wxyz"
    };

public:
    vector<string> letterCombinations(string digits) {
        if (digits == "") return {};

        std::vector<string> result = {""};

        for (char c : digits) {
            int num = c - '0';
            std::vector<string> tmp = {};
            for (string str : result) {
                for (char l : map_of_phone[num]) {
                    tmp.push_back(str+l);
                }
            }
            result = tmp;
        }

        return result;
    }
};