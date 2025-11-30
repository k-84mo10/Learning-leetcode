impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let days = temperatures.len();

        let mut answer = vec![0i32; days];
        let mut stack = Vec::new();

        for day in 0..days {
            let temperature = temperatures[day];

            while let Some(&idx) = stack.last() {
                if temperatures[idx] < temperature {
                    answer[idx] = (day - idx) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push(day);
        }

        answer
    }
}