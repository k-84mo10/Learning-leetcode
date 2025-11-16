use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let cols = grid.len();
        let rows = grid[0].len();

        let mut rotten_map: VecDeque<(usize, usize)> = VecDeque::new(); 
        let mut num_fresh = 0;

        for col in 0..cols {
            for row in 0..rows {
                let cell = grid[col][row];
                match cell {
                    1 => num_fresh += 1,
                    2 => rotten_map.push_back((col, row)),
                    _ => {}
                }
            }
        }

        let mut minutes = 0;
        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        while !rotten_map.is_empty() {
            let rotten_num = rotten_map.len();
            for _ in 0..rotten_num {
                if let Some((col, row)) = rotten_map.pop_front() {
                    for (dc, dr) in dirs {
                        let nc = col as isize + dc;
                        let nr = row as isize + dr;

                        if 0 <= nc && nc < cols as isize && 0 <= nr && nr < rows as isize {
                            let (nc, nr) = (nc as usize, nr as usize);

                            if grid[nc][nr] == 1 {
                                grid[nc][nr] = 2;
                                rotten_map.push_back((nc, nr));
                                num_fresh -= 1;
                            }
                        }
                    }
                }
            }
            
            if !rotten_map.is_empty() {
                minutes += 1;
            }
        }

        // fresh orange が無いか確認
        if num_fresh != 0 {
            return -1;
        }

        minutes
    }
}