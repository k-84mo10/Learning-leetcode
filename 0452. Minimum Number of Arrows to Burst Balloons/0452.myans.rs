impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_by_key(|point| point[1]);

        let mut arrows = 1;
        let mut last_target = points[0][1];

        for point in points.iter().skip(1) {
            let start = point[0];
            let end = point[1];

            if last_target < start {
                arrows += 1;
                last_target = end;
            } 
        }
        
        arrows
    }
}