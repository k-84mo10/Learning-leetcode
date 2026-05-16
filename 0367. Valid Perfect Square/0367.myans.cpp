class Solution {
public:
    bool isPerfectSquare(int num) {
        long left = 0;
        long right = num;

        while (left <= right) {
            long mid = left + (right - left) / 2;
            long square = mid * mid;
            if (num == square) {
                return true;
            } else if (num < square) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        return false;
    }
};