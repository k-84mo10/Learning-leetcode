class Solution {
    bool canRepairCarsInTime(vector<int>& ranks, int cars, long long time) {
        int sum = 0;
        for (int& rank : ranks) {
            sum += std::sqrt(time / rank);
            if (sum >= cars) return true;
        }
        return false;
    }

public:
    long long repairCars(vector<int>& ranks, int cars) {
        // 最小のrankを求める
        int minRank = *std::min_element(ranks.begin(), ranks.end());
        // ranksが空配列になり、rank.end()が返ってくることは、今回は制約上ないため、例外処理はしない

        // 最小のrankのみが行った場合の時間を最大値とし、二分探索で最小値を求める
        long long left = 0, right = static_cast<long long>(minRank)*cars*cars;
        while (left < right) {
            long long middle = (left + right) / 2;
            if (canRepairCarsInTime(ranks, cars, middle)) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        return left;
    }
};