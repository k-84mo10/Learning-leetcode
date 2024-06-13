class Solution {
public:
    void generate(std::vector<std::string>& res, int l, int r, string s) {
        if (l == 0 && r == 0) {
            res.push_back(s);
            return;
        }

        if (l) {
            s += "(";
            generate(res, l-1, r, s);
            s.pop_back();
        }

        if (r > l) {
            s += ")";
            generate(res, l, r-1, s);
        }
    }
    vector<string> generateParenthesis(int n) {
        std::vector<std::string> res;
        generate(res, n-1, n, "(");
        return res;
    }
};