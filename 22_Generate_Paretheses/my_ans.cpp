class Solution {
    vector<string> ans;

public:
    vector<string> generateParenthesis(int n) {
        dfs(n, n, "");
        return ans;
    }

    void dfs(int left_num, int right_num, string parenthese) {
        if (left_num < 0 || right_num < 0 || left_num > right_num) return ;

        if (left_num == 0 && right_num == 0) {
            ans.push_back(parenthese);
            return ;
        }

        dfs(left_num-1, right_num, parenthese+'(');
        dfs(left_num, right_num-1, parenthese+')');
    }
};