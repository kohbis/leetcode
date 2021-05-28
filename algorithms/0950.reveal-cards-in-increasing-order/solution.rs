impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut res = VecDeque::new();

        let mut cards = deck.clone();
        cards.sort_unstable();
        cards.reverse();

        for i in 0..cards.len() {
            if res.len() > 0 {
                let last = res.pop_back().unwrap();
                res.push_front(last);
            }
            res.push_front(cards[i]);
        }

        Vec::from(res)
    }
}
