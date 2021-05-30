impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        fn str_to_num(word: &str) -> u8 {
            let mut num = 0;
            for b in word.as_bytes() {
                num = num * 10 + (b - 97);
            }
            num
        }
        str_to_num(&first_word) + str_to_num(&second_word) == str_to_num(&target_word)
    }
}
