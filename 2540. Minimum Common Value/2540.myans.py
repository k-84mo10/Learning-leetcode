class Solution:
    def getCommon(self, nums1: List[int], nums2: List[int]) -> int:
        len1 = len(nums1)
        len2 = len(nums2)

        idx1 = 0
        idx2 = 0

        while idx1 < len1 and idx2 < len2:
            n1 = nums1[idx1]
            n2 = nums2[idx2]

            if n1 == n2:
                return n1
            elif n1 < n2:
                idx1 += 1
            else:
                idx2 += 1
        
        return -1