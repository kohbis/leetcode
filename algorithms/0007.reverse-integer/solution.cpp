#include <climits>

class Solution {
public:
    int reverse(int x) {
        long long res = 0;

        while (x != 0) {
            res = res * 10 + x % 10;
            x /= 10;
        }

        if (res > INT_MAX || INT_MIN > res) {
            return 0;
        } else {
            return res;
        }
    }
};
