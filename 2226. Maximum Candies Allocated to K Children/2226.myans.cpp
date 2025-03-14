class Solution {
    // divide_numずつ配った際にk人に配れるかを調べる関数
    bool canDivideCandies(vector<int>& candies, long long k, int divide_num) {
        long long count = 0;
        for (int& candy : candies) {
            count += candy / divide_num;
            if (count >= k) return true;
        }
        return false;
    }

public:
    int maximumCandies(vector<int>& candies, long long k) {
        // パイルの最大値を線型探索
        int max_candy_pile = 0;
        for (int& candy : candies) {
            max_candy_pile = max(candy, max_candy_pile);
        }

        // 二分探索で渡せるキャンディの最大値を探す
        int left = 1, right = max_candy_pile;
        int answer = 0;
        while (left <= right) {
            int middle = (left + right) / 2;
            if (canDivideCandies(candies, k, middle)) {
                answer = middle;
                left = middle + 1;
            } else {
                right = middle - 1;
            }
        }
        return answer;
    }
};
