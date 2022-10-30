impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        if event1[0] <= event2[1] {
            return event1[1] >= event2[0];
        } else {
            return event1[1] <= event2[0];
        }
    }
}
