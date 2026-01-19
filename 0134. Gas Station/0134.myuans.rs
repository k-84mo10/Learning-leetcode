impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut start = 0usize;
        let mut tank = 0;
        let mut total = 0;

        for (idx, (g, c)) in gas.iter().zip(cost.iter()).enumerate() {
            let diff = g - c;
            tank += diff;
            total += diff;
            if tank < 0 {
                start = idx + 1;
                tank = 0;
            }
        }

        if total >= 0 {
            start as i32
        } else {
            -1
        }
    }
}