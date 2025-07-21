func exist(board [][]byte, word string) bool {
    m := len(board)
    n := len(board[0])

    passed := make([][]bool, m)
    for i := range passed {
        passed[i] = make([]bool, n)
    }

    for i:=0; i<m; i++ {
        for j:=0; j<n; j++ {
            if dfs(board, word, i, j, 0, passed) {
                return true
            }
        }
    }

    return false
}

func dfs(board [][]byte, word string, i int, j int, index int, passed [][]bool) bool{
    if board[i][j] != word[index] || passed[i][j] {
        return false
    } 

    passed[i][j] = true
    if index == len(word)-1 {
        return true
    }

    if i != 0 && dfs(board, word, i-1, j, index+1, passed) {
        return true
    }
    if i != len(board)-1 && dfs(board, word, i+1, j, index+1, passed) {
        return true
    }
    if j != 0 && dfs(board, word, i, j-1, index+1, passed) {
        return true
    }
    if j != len(board[0])-1 && dfs(board, word, i, j+1, index+1, passed) {
        return true
    }

    passed[i][j] = false
    return false
}