func canVisitAllRooms(rooms [][]int) bool {
    n := len(rooms)
    canEnterRoom := make([]bool, n)
    stack := []int{0}

    for len(stack) > 0 {
        roomNum := stack[len(stack)-1]
        stack = stack[:len(stack)-1]

        if !canEnterRoom[roomNum] {
            canEnterRoom[roomNum] = true
            for _, room := range rooms[roomNum] {
                stack = append(stack, room)
            }
        }
    }

    for i := 0; i < n; i++ {
        if !canEnterRoom[i] {
            return false
        }
    }

    return true
}