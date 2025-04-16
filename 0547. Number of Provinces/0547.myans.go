func findCircleNum(isConnected [][]int) int {
    n := len(isConnected)
    isDetected := make([]bool, n)
    province := 0
    for i := 0; i < n; i++ {
        if !isDetected[i] {
            province += 1
            isDetected[i] = true;
            dfs(i, n, isConnected, isDetected)
        }
    }
    return province
}

func dfs(index int, n int, isConnected [][]int, isDetected []bool) {
    for i := 0; i < n; i++ {
        if isConnected[index][i] == 1 && !isDetected[i] {
            isDetected[i] = true;
            dfs(i, n, isConnected, isDetected)
        }
    }
}