class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        num_set = set()
        for num in nums:
            num_set.add(num)

        longest_stream = 0

        for num in num_set:
            if num-1 in num_set:
                continue
            current_stream = 1
            current_num = num + 1
            while current_num in num_set:
                current_stream += 1
                current_num += 1
            longest_stream = max(longest_stream, current_stream)
        
        return longest_stream
            