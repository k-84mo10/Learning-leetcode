impl Solution {
    pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut schedule: Vec<i32> = vec![0; (days+2) as usize];
        let mut count = 0;

        for meeting in meetings {
            let start = meeting[0] as usize;
            let end = meeting[1] as usize;
            schedule[start] += 1;
            schedule[end+1] -= 1;
        }

        let mut sum = 0;
        for i in 1..days+1 {
            sum += schedule[i as usize];
            if sum == 0 {
                count += 1;
            }
        }

        count
    }
}