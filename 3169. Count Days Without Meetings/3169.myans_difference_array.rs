use std::collections::BTreeMap;

impl Solution {
    pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut day_map = BTreeMap::new();

        for meeting in meetings {
            let start = meeting[0];
            let end = meeting[1];
            
            *day_map.entry(start).or_insert(0) += 1;
            *day_map.entry(end+1).or_insert(0) -= 1;
        }

        let mut free_day = 0;
        let mut prev_day = 1;
        let mut ongoing = 0;

        for (&day, &delta) in &day_map {
            if ongoing == 0 && day > prev_day {
                free_day += day - prev_day;
            }

            prev_day = day;
            ongoing += delta;
        }

        if prev_day <= days && ongoing == 0 {
            free_day += days - prev_day + 1;
        }

        free_day
    }
}