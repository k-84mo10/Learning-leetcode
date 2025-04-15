func countBits(n int) []int {
    bitSum := make([]int, n+1)
    largestDigit := 0

    for i := 1; i <= n; i++ {
        if isPowerOfTwo(i) {
            bitSum[i] = 1;
            largestDigit = i
        } else {
            bitSum[i] = bitSum[i-largestDigit] + 1;
        }
    }

    return bitSum
}

func isPowerOfTwo(n int) bool {
    return n != 0 && (n & (n-1) == 0)
}