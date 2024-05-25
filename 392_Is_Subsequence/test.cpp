# include <iostream>
#include <string>
using namespace std;

bool isSubsequence(string s, string t) {
    cout << "s.size() = " << s.size() << endl;
    cout << "t.size() = " << t.size() << endl;
    int ptr = 0;
    if (t.size() == 0 && s.size() != 0) return false;
    for (int i = 0; i < s.size(); i++) {
        if (ptr == t.size()) return false;
        for (int j = ptr; j < t.size(); j++) {
            if (s[i] == t[j]) {
                ptr = j + 1;
                break;
            }
            if (j == t.size() - 1) return false;
        }
    } 
    return true;
}

int main() {
    string s = "acb";
    string t = "ahbgdc";
    std::cout << isSubsequence(s, t) << std::endl;
}