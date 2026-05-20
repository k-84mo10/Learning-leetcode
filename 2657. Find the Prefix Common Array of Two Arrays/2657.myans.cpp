class Solution {
public:
    vector<int> findThePrefixCommonArray(vector<int>& A, vector<int>& B) {
        std::set<int> exists;
        std::vector<int> prefixCommonArray(A.size());
        int common = 0;

        for (int i = 0; i < A.size(); i++) {
            if (!exists.insert(A[i]).second) {
                common += 1;
            }

            if (!exists.insert(B[i]).second) {
                common += 1;
            }

            prefixCommonArray[i] = common;
        }

        return prefixCommonArray;
    }
};