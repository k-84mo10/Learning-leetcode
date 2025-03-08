class Solution {
public:
    int minimumRecolors(string blocks, int k) {
        // Count the number of 'B' in the first k-length window
        int black_num = 0;
        for (int i = 0; i < k; i++) {
            if (blocks[i] == 'B') black_num++;
        }
        int max_black_num = black_num;

        // Use a sliding window to find the maximum number of 'B' in any k-length segment
        for (int i = k; i < blocks.size(); i++) {
            // Remove the leftmost element from the window (decrease count if it was 'B')
            if (blocks[i - k] == 'B') black_num--;
            // Add the new rightmost element to the window (increase count if it is 'B')
            if (blocks[i] == 'B') black_num++;
            // Update the maximum number of 'B' found in any k-length window
            max_black_num = max(black_num, max_black_num);
        }

        // The minimum number of 'W' to recolor is the difference between k and max_black_num
        return k - max_black_num;
    }
};

