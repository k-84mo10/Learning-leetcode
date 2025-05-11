func threeConsecutiveOdds(arr []int) bool {
    count := 0
    for _, x := range arr {
        if x % 2 == 1 {
            count += 1
            if count == 3 { return true }
        } else {
            count = 0
        }
    }
    return false 
}