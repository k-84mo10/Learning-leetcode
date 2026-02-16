use std::collections::HashMap;

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;

        for (i, p) in points.iter().enumerate() {
            let mut freq: HashMap<i32, i32> = HashMap::new();

            let (px, py) = (p[0], p[1]);

            for (j, q) in points.iter().enumerate().filter(|(j, _)| *j != i) {
                let distance_x = q[0] - px;
                let distance_y = q[1] - py;
                let distance = distance_x * distance_x + distance_y * distance_y;
                *freq.entry(distance).or_insert(0) += 1;
            }

            for v in freq.values() {
                if *v >= 2 {
                    count += v * (v - 1);
                }
            }
        }

        count
    }
}