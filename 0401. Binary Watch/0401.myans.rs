impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut possible_time: Vec<String> = Vec::new();
        
        for h in 0..12 {
            let h_bitcount = (h as usize).count_ones() as i32;
            for m in 0..60 {
                let m_bitcount = (m as usize).count_ones() as i32;
                if h_bitcount + m_bitcount == turned_on {
                    let time: String = if m / 10 == 0 {
                        format!("{}:0{}", h, m)
                    } else {
                        format!("{}:{}", h, m)
                    };
                    possible_time.push(time);
                }
            }
        }

        possible_time
    }
}