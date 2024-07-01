class Solution {
public:
    bool isValid(string s) {
        stack<char> order_to_close;
        unordered_map<char, char> compatible_bracket = {{')','('}, {'}','{'}, {']','['}};

        for (char& char_ : s) {
            if (compatible_bracket.find(char_) != compatible_bracket.end()) {
                if (order_to_close.empty() || order_to_close.top() != compatible_bracket[char_]) return false;
                order_to_close.pop();
            } else {
                order_to_close.push(char_);
            }
        }

        if (order_to_close.empty()) {
            return true;
        } else {
            return false;
        }
    }
};