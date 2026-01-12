class Solution:
    def minTimeToVisitAllPoints(self, points: List[List[int]]) -> int:
        if len(points) == 1:
            return 0
        
        time = 0
        
        for i in range(0, len(points)-1):
            time += self.minTimeToVisitNextPoint(points[i], points[i+1])

        return time 

    def minTimeToVisitNextPoint(self, start: List[int], end: List[int]) -> int:
        start_x = start[0]
        start_y = start[1]
        end_x = end[0]
        end_y = end[1]

        x_diff = abs(end_x - start_x)
        y_diff = abs(end_y - start_y)

        return max(x_diff, y_diff)