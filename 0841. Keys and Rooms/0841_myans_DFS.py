from queue import Queue


class Solution:
    def canVisitAllRooms(self, rooms: List[List[int]]) -> bool:
        have_room_visited = [False] * len(rooms)
        q = Queue()

        q.put(0)

        while q.qsize() > 0:
            room_num = q.get()

            if have_room_visited[room_num] == False:
                have_room_visited[room_num] = True
                for room in rooms[room_num]:
                    q.put(room)

        for i in range(len(rooms)):
            if have_room_visited[i] == False:
                return False

        return True
