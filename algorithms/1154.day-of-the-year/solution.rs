impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let year_month_day: Vec<i32> = date.split("-").map(|s| s.parse::<i32>().unwrap()).collect();
        let mut days: Vec<i32> = vec![
            0, 31, // Jan.
            28, // Feb.
            31, // Mar.
            30, // Apr.
            31, // May.
            30, // June
            31, // July
            31, // Aug.
            30, // Sept.
            31, // Oct.
            30, // Nov.
            31, // Dec.
        ];

        if Self::leap_year(year_month_day[0]) {
            days[2] = 29;
        }

        let mut sum: i32 = 0;
        for i in 0..year_month_day[1] {
            sum += days[i as usize]
        }

        sum + year_month_day[2]
    }

    fn leap_year(year: i32) -> bool {
        (year % 4 == 0) && !(year % 100 == 0) || (year % 400 == 0)
    }
}
