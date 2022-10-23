impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let (mut t_metal, mut t_paper, mut t_glass) = (0, 0, 0);

        let mut total_time = 0i32;
        for (i, t) in travel.iter().enumerate() {
            total_time += t;
            if garbage[i + 1].contains('M') {
                t_metal = total_time;
            }
            if garbage[i + 1].contains('P') {
                t_paper = total_time;
            }
            if garbage[i + 1].contains('G') {
                t_glass = total_time;
            }
        }

        garbage.join("").len() as i32 + t_metal + t_paper + t_glass
    }
}
