func isUgly(n int) bool {
    if n <= 0 {
        return false
    }

    n = divideOut(n, 2)
    n = divideOut(n, 3)
    n = divideOut(n, 5)

    return n == 1
}

func divideOut(n, d int) int {
    for n % d == 0 {
        n /= d
    }
    return n
}