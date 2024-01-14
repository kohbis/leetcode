impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut count1 = [0; 26];
        let mut count2 = [0; 26];
        for c in word1.chars() {
            count1[(c as u8 - b'a') as usize] += 1;
        }
        for c in word2.chars() {
            count2[(c as u8 - b'a') as usize] += 1;
        }

        for i in 0..26 {
            if ((count1[i] == 0) && (count2[i] != 0)) || ((count1[i] != 0) && (count2[i] == 0)) {
                return false;
            }
        }

        count1.sort_unstable();
        count2.sort_unstable();
        for i in 0..26 {
            if count1[i] != count2[i] {
                return false;
            }
        }

        true
    }
}
