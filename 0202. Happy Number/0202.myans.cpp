class Solution {
public:
    bool isHappy(int n) {
        std::set<int> seen;
        int base = n;
        while (true) {
            if (base == 1) return true;

            auto [it, inserted] = seen.insert(base);
            if (!inserted) return false;

            base = calcSquareSum(base);
        }
    }

    int calcSquareSum(int n) {
        int sum = 0;
        int base = n;
        while (base != 0) {
            int num = base % 10;
            sum += num * num;
            base /= 10;
        }
        return sum;
    }
};