class Solution {
public:
    int divide(int dividend, int divisor) {
        // INT_MINは符号反転するとオーバーフローするため、ここで例外処理
        if (dividend == INT_MIN) {
            if (divisor == 1) return INT_MIN;
            if (divisor == -1) return INT_MAX;
        }

        long long ll_dividend = static_cast<long long>(dividend);
        long long ll_divisor = static_cast<long long>(divisor);

        long long answer = 0; 

        // 商の符号を決定
        bool is_negative = (ll_dividend < 0) ^ (ll_divisor < 0);

        // それぞれの数の絶対値で計算
        ll_dividend = llabs(ll_dividend);
        ll_divisor = llabs(ll_divisor);

        // シフトして計算
        while (ll_dividend >= ll_divisor) {
            long long tmp = ll_divisor, multiple = 1;
            while (ll_dividend > (tmp << 1)) {
                tmp <<= 1;
                multiple <<= 1;
            } 
            ll_dividend -= tmp;
            answer += multiple;
        }

        return is_negative ? -1*static_cast<int>(answer) : static_cast<int>(answer);
    }
};