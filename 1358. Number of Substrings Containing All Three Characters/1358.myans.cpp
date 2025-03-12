class Solution {
public:
    int numberOfSubstrings(string s) {
        int latest_a = -1, latest_b = -1, latest_c = -1;
        int minimum_line = -1;
        int count = 0;

        for (int i = 0; i < s.size(); i++) {
            // record each latest
            if (s[i] == 'a') latest_a = i;
            if (s[i] == 'b') latest_b = i;
            if (s[i] == 'c') latest_c = i;

            // calc minimum_line
            minimum_line = min(latest_a, min(latest_b, latest_c));
            if (minimum_line == -1) continue;

            count += minimum_line+1;
        }
        return count;
    }
};
