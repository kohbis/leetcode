impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort_unstable();

        let (mut left, mut right) = (0, people.len() - 1);
        let mut count = 0i32;

        while left < right {
            if people[left] + people[right] <= limit {
                left += 1;
            }
            right -= 1;
            count += 1;
        }

        if left == right {
            count += 1;
        }

        count
    }
}
