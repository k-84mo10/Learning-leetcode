
class Solution {
    vector<string> res;
public:
    vector<string> generateParenthesis(int n) {
        generate(n, n, "");
        return res;
    }

    void generate (int left, int right, string s) {
        if (left == 0 && right == 0) {
            res.push_back(s);
            return ;
        }

        if (left) {
            string new_s = s +'(';
            generate(left-1, right, new_s);
        }

        if (right > left) {
            string new_s = s + ')';
            generate(left, right-1, new_s);
        }
    }
};