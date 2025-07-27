class Solution:
    def countHillValley(self, nums: List[int]) -> int:
        i = 0
        while i < len(nums)-1 and nums[i] == nums[i+1]:
            i += 1

        if i >= len(nums)-2:
            return 0

        up_flag = False
        if nums[i] < nums[i+1]:
            up_flag = True
        
        count = 0
        i += 1
        while i < len(nums):
            if up_flag and nums[i-1] > nums[i]:
                count += 1
                up_flag = False
            elif up_flag is False and nums[i-1] < nums[i]:
                count += 1
                up_flag = True
            i += 1
        
        return count