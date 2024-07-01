class Solution {
public:
    int kthGrammar(int n, int k) {
        int inversion = 0;
        k -= 1;
        for (int i = 0; i < n; i++) {
            if (k % 2 == 1) inversion++;
            k = parent(k);
            if (k == 0) break;
        }
        return (inversion % 2 == 0) ? 0 : 1;
    }

    int parent (int k) {
        return k / 2;
    }
};