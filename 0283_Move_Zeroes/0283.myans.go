func moveZeroes(nums []int)  {
    left := 0
    right := 0

    for right < len(nums) {
        if nums[right] != 0 {
            nums[left] = nums[right]
            left += 1
        }
        right += 1
    }

    for left != right {
        nums[left] = 0
        left += 1
    }
}