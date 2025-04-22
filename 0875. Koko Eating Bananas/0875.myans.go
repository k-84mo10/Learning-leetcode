func minEatingSpeed(piles []int, h int) int {
    left := 1
    right := 1_000_000_000

    for left < right {
        mid := left + (right - left) / 2
        if canEatBanana(piles, h, mid) {
            right = mid
        } else {
            left = mid + 1
        }
    }
    return left
}

func canEatBanana(piles []int, h int, k int) bool {
    time := 0
    for _, pile := range piles {
        time += (pile - 1) / k + 1
        if h < time {
            return false
        }
    }
    return true
}