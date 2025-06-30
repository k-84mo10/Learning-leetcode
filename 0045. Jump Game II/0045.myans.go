func jump(nums []int) int {
    // n 回目のjumpでどこまでいけるか、を考える
    jump_count := 0
    cur_farthest := 0
    farthest := 0

    for i := 0; i < len(nums); i++ {
        if i + nums[i] > farthest {
            farthest = i + nums[i]
        }

        if i == cur_farthest {
            jump_count += 1
            cur_farthest = farthest
        }
    }

    return jump_count
}