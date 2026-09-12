use std::collections::VecDeque;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = vec![Vec::new(); num_courses as usize];
        let mut indegrees = vec![0; num_courses as usize];

        for p in prerequisites {
            let course = p[0] as usize;
            let prerequisite = p[1] as usize;

            graph[prerequisite].push(course);
            indegrees[course] += 1;
        }

        let mut queue = VecDeque::new();
        let mut order = Vec::new();

        for (idx, indegree) in indegrees.iter().enumerate() {
            if *indegree == 0 {
                queue.push_back(idx);
            }
        }

        while let Some(course) = queue.pop_front() {
            order.push(course as i32);

            for &next in &graph[course] {
                indegrees[next] -= 1;

                if indegrees[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        if order.len() == num_courses as usize {
            order
        } else {
            Vec::new()
        }
    }
}