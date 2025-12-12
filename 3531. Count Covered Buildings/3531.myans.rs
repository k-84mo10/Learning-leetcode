impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut row_min = vec![i32::MAX; n+1];
        let mut row_max = vec![i32::MIN; n+1];
        let mut col_min = vec![i32::MAX; n+1];
        let mut col_max = vec![i32::MIN; n+1];

        for building in buildings.iter() {
            let (x, y) = (building[0], building[1]);
            let (xi, yi) = (x as usize, y as usize);

            row_min[yi] = row_min[yi].min(x);
            row_max[yi] = row_max[yi].max(x);
            col_min[xi] = col_min[xi].min(y);
            col_max[xi] = col_max[xi].max(y); 
        }

        let mut covered = 0;

        for building in buildings.iter() {
            let (x, y) = (building[0], building[1]);
            let (xi, yi) = (x as usize, y as usize);

            if row_min[yi] < x && row_max[yi] > x && col_min[xi] < y && col_max[xi] > y {
                covered += 1;
            }
        }

        covered
    }
}