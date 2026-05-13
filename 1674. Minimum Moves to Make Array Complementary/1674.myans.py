class Solution:
    def minMoves(self, nums: List[int], limit: int) -> int:
        diff = [0]*(2*limit+2)
        n = len(nums)

        for i in range(n//2):
            a = min(nums[i], nums[n-1-i])
            b = max(nums[i], nums[n-1-i])

            diff[2] += 2
            diff[a+1] -= 1
            diff[a+b] -= 1
            diff[a+b+1] += 1
            diff[b+limit+1] += 1

        current_ops = 0
        min_ops = float("inf")

        for i in range(2, 2*limit+1):
            current_ops += diff[i]
            min_ops = min(min_ops, current_ops)

        return min_ops