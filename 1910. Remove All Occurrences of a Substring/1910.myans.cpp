class Solution {
    public:
        string removeOccurrences(string s, string part) {
            std::string answer;
            const int part_size = part.size();
            
            for (char& c : s) {
                answer += c;
                if (answer.size() >= part_size) {
                    if (answer.substr(answer.size()-part_size, part_size) == part) {
                        answer.erase(answer.size()-part_size, part_size);
                    }
                }
            }
    
            return answer;
        }
    };