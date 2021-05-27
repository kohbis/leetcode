impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut visited = vec![false; n as usize];
        for edge in edges {
            visited[edge[1] as usize] = true;
        }

        visited
            .into_iter()
            .enumerate()
            .filter(|(_, v)| !v)
            .map(|(i, _)| i as i32)
            .collect()
    }
}
