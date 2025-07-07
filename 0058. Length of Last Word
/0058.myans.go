func lengthOfLastWord(s string) int {
    s = strings.TrimRight(s, " ")

    count := 0
    for i := len(s)-1; i >= 0; i-- {
        if s[i] == ' ' { break }
        count ++
    }

    return count
}