func findThePrefixCommonArray(A []int, B []int) []int {
    n := len(A)
    exists := make(map[int]bool)
    common := 0
    answer := make([]int, n)

    for i := range A {
        x := A[i]
        y := B[i]

        if exists[x] {
            common += 1
        }
        exists[x] = true

        if exists[y] {
            common += 1
        }
        exists[y] = true

        answer[i] = common
    }

    return answer
}