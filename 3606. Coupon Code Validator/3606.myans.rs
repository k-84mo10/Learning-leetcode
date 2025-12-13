impl Solution {
    pub fn validate_coupons(code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
        
        fn rank(bl: &str) -> Option<u8> {
            match bl {
                "electronics" => Some(0),
                "grocery" => Some(1),
                "pharmacy" => Some(2),
                "restaurant" => Some(3),
                _ => None,
            }
        }

        fn valid_code(s: &str) -> bool {
            !s.is_empty() && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
        }

        let mut valid_coupons: Vec<(u8, String)> = Vec::new();

        for ((c, bl), &active) in code.iter().zip(business_line.iter()).zip(is_active.iter()) {
            if !active { continue; }
            let Some(r) = rank(bl) else { continue };
            if !valid_code(c) { continue; }

            valid_coupons.push((r, c.clone()));
        }

        valid_coupons.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        valid_coupons.into_iter().map(|(_, c)| c).collect()
    }
}