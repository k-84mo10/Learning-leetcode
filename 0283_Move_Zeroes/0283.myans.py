class Solution:
    def moveZeroes(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        left = 0
        right = 0

        while right < len(nums) :
            if nums[right] != 0 :
                nums[left] = nums[right]
                left += 1
            right += 1

        while left != right :
            nums[left] = 0
            left += 1