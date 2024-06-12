class Solution {
public:
    bool isValid(string s) {
        stack<int> order_to_close;  // ( => 0; { => 1; [ => 2;

        for (int i = 0; i < s.size(); i++) {
            switch(s[i]) {
                case ('(') : {
                    order_to_close.push(0);
                    break;
                }
                case ('{') : {
                    order_to_close.push(1);
                    break;
                }
                case ('[') : {
                    order_to_close.push(2);
                    break;
                }
                case (')') : {
                    if (order_to_close.empty()) return false;
                    if (order_to_close.top() != 0) return false;
                    order_to_close.pop();
                    break;
                }
                case ('}') : {
                    if (order_to_close.empty()) return false;
                    if (order_to_close.top() != 1) return false;
                    order_to_close.pop();
                    break;
                }
                case (']') : {
                    if (order_to_close.empty()) return false;
                    if (order_to_close.top() != 2) return false;
                    order_to_close.pop();
                    break;
                }
                default : {
                    cout << "s has unexpected character." << endl;
                    return false;
                }
            }
        }

        if (order_to_close.empty()) {
            return true;
        } else {
            return false;
        }
    }
};