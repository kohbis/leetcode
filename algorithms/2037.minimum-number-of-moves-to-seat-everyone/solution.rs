impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let (mut seats, mut students) = (seats, students);
        seats.sort_unstable();
        students.sort_unstable();

        seats
            .iter()
            .zip(students.iter())
            .map(|(seat, student)| (seat - student).abs())
            .sum()
    }
}
