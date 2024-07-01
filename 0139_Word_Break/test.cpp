#include <iostream>
#include <vector>
#include <string>
using namespace std;

class Solution {
public:
    bool wordBreak(string s, vector<string>& wordDict) {
        int left = 0; 
        int right = 0;
        int n = s.size();
        while (right < n+1) {
            for (string& word: wordDict) {
                string current_s = s.substr(left, right);
                if (current_s == word) {
                    cout << "current_s=" << current_s << endl;
                    left = left + right;
                    right = 0;   
                    if (left == n) return true;
                }
            }
            right += 1;
        }
        left = 0;
        right = n;
        while (right > 0) {
            for (string& word: wordDict) {
                string current_s = s.substr(left, right);
                if (current_s == word) {
                    cout << "current_s=" << current_s << endl;
                    left = left + right;
                    right = n-left;   
                    if (left == n) return true;
                }
            }
            cout << "right=" << right << endl;
            right -= 1;
        }
        return false;
    }
};

int main() {
    Solution sol;
    string s = "aaaaaaa";
    vector<string> wordDict = {"aaaa", "aaa"};
    cout << sol.wordBreak(s, wordDict) << endl;
    return 0;
}