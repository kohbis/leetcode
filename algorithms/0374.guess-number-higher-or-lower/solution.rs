/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is lower than the guess number
 *			      1 if num is higher than the guess number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let (mut left, mut right) = (0, n);
        let mut mid: i32;

        loop {
            mid = left + (right - left) / 2;
            match guess(mid) {
                -1 => { right = mid - 1; },
                1 => { left  = mid + 1; },
                _ => { return mid; },
            }
        }
    }
}
