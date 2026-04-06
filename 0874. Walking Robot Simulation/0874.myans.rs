use std::collections::HashSet;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let obstacles_set: HashSet<(i32, i32)> = obstacles
            .into_iter()
            .map(|v| (v[0], v[1]))
            .collect();

        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut dir = 0usize;
        let mut current_step = (0, 0);
        let mut max_square = 0;

        for command in commands {
            if command == -1 {
                dir = (dir + 1) % 4;
                continue;
            }

            if command == -2 {
                dir = (dir + 3) % 4;
                continue;
            }

            let (dx, dy) = dirs[dir];

            for step in 0..command as usize {
                let next_step = (
                    current_step.0 + dx,
                    current_step.1 + dy
                );

                if obstacles_set.contains(&next_step) { break; }

                current_step = next_step;
            }

            let current_square = current_step.0 * current_step.0 + current_step.1 * current_step.1;
            max_square = max_square.max(current_square);
        }

        max_square
    }
}