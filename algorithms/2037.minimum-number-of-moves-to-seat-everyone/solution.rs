impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let (mut seats, mut students) = (seats, students);
        seats.sort_unstable();
        students.sort_unstable();

        let mut res = 0i32;
        for i in 0..seats.len() {
            res += (seats[i] - students[i]).abs();
        }

        res
    }
}
