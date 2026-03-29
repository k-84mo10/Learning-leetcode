impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();

        let mut even1 = [s1[0], s1[2]];
        let mut even2 = [s2[0], s2[2]];
        let mut odd1 = [s1[1], s1[3]];
        let mut odd2 = [s2[1], s2[3]];

        even1.sort_unstable();
        even2.sort_unstable();
        odd1.sort_unstable();
        odd2.sort_unstable();

        even1 == even2 && odd1 == odd2
    }
}