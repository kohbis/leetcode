impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut people: Vec<(String, i32)> = names.into_iter().zip(heights.into_iter()).collect();
        people.sort_by_key(|(_, height)| -height);
        people.into_iter().map(|x| x.0).collect()
    }
}
