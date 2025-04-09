class Solution:
    def canJump(self, nums: List[int]) -> bool:
        if len(nums) == 1:
            return True

        far_distance = 0

        for i in range(0, len(nums)-1):
            if far_distance < i:
                return False
            
            far_distance = max(far_distance, i+nums[i])
            if far_distance >= len(nums)-1:
                return True
        
        return False