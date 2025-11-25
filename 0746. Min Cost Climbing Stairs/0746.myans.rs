impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let stairs = cost.len();

        match stairs {
            1 => return 0,
            2 => return cost[0].min(cost[1]),
            _ => {}
        }

        let (mut one_step_before, mut two_step_before) = (cost[1], cost[0]);

        for i in 2..stairs {
            let current = cost[i] + one_step_before.min(two_step_before);
            two_step_before = one_step_before;
            one_step_before = current;
        }

        one_step_before.min(two_step_before)
    }
}