use std::collections::VecDeque;
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        // ある都市から出る道を[目的の都市番号、向き（trueなら都市に来る、falseなら都市から出る）]として表記する。
        let mut load_map: Vec<VecDeque<(usize, bool)>> = vec![VecDeque::new(); n];
        for connection in connections {
            let source = connection[0] as usize;
            let destination = connection[1] as usize;
            load_map[source].push_back((destination, false));
            load_map[destination].push_back((source, true));
        }

        let mut is_detected: Vec<bool> = vec![false; n];
        is_detected[0] = true;
        Self::dfs(0, &load_map, &mut is_detected)
    }

    fn dfs(index: usize, load_map: &Vec<VecDeque<(usize, bool)>>, is_detected: &mut Vec<bool>) -> i32 {
        let mut reverse = 0;
        for &(city, is_dist) in &load_map[index] {
            if is_detected[city] {
               continue; 
            }

            is_detected[city] = true;
            if !is_dist {
                reverse += 1;
            }
            reverse += Self::dfs(city, &load_map, is_detected);
        }
        reverse
    }
}