func minMoves(nums []int, limit int) int {
    diff := make([]int, 2*limit+2)
    n := len(nums)

    for i := 0; i < (n+1)/2; i++ {
        a := min(nums[i], nums[n-1-i])
        b := max(nums[i], nums[n-1-i])

        diff[2] += 2
        diff[a+1] -= 1
        diff[a+b] -= 1
        diff[a+b+1] += 1
        diff[b+limit+1] += 1
    }

    min_ops := math.MaxInt
    current_ops := 0

    for i := 2; i < 2*limit+2; i++ {
        current_ops += diff[i]
        min_ops = min(min_ops, current_ops)
    }

    return min_ops
}