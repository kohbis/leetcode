impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut total: i32 = 0;
        let week = (n / 7) as i32;
        let extra_day = (n % 7) as i32;

        total += 28 * week;
        total += week * (week - 1) / 2 * 7;
        total += week * extra_day;
        total += extra_day * (extra_day + 1) / 2;

        total
    }
}
