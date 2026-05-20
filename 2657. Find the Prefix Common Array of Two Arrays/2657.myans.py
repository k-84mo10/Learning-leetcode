class Solution:
    def findThePrefixCommonArray(self, A: List[int], B: List[int]) -> List[int]:
        exists = set()
        common = 0
        ans = [0] * len(A)

        for i, (x, y) in enumerate(zip(A, B)):
            if x in exists:
                common += 1
            else:
                exists.add(x)

            if y in exists:
                common += 1
            else:
                exists.add(y)
        
            ans[i] = common

        return ans