func isPerfectSquare(num int) bool {
    left := 1
    right := num

    for left <= right {
        mid := left + (right - left) / 2
        square := mid * mid
        if num == square {
            return true
        } else if num < square {
            right = mid - 1
        } else {
            left = mid + 1
        }
    }

    return false
}