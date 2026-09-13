impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len() as i32;

        let mut ones1 = Vec::new();
        let mut ones2 = Vec::new();

        for (i, row) in img1.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                if x == 1 {
                    ones1.push((i as i32, j as i32));
                }
            }
        }

        for (i, row) in img2.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                if x == 1 {
                    ones2.push((i as i32, j as i32));
                }
            }
        }

        let size = (2 * n - 1) as usize;
        let offset = n - 1;

        let mut distances = vec![0; size * size];
        let mut answer = 0;

        for &(x1, y1) in &ones1 {
            for &(x2, y2) in &ones2 {
                let dx = x2 - x1 + offset;
                let dy = y2 - y1 + offset;

                let index = dx as usize * size + dy as usize;

                distances[index] += 1;
                answer = answer.max(distances[index]);
            }
        }

        answer
    }
}