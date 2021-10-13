/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is lower than the guess number
 *			      1 if num is higher than the guess number
 *               otherwise return 0
 * int guess(int num);
 */

class Solution {
  public:
    int guessNumber(int n) {
        int left = 0;
        int right = n;

        int res;
        while (left <= right) {
            int mid = left + (right - left) / 2;

            int pick = guess(mid);
            if (pick == -1) {
                right = mid - 1;
            } else if (pick == 1) {
                left = mid + 1;
            } else {
                res = mid;
                break;
            }
        }

        return res;
    }
};
