#include <vector>

using namespace std;

class Solution {
public:
    int climbStairs(int n) {
        vector<int> ways(n, 0);

        for (int i = 0; i < n; i++) {
            if (i == 0) {
                ways[i] = 1;
            } else if (i == 1) {
                ways[i] = 2;
            } else {
                ways[i] = ways[i - 1] + ways[i - 2];
            }
        }

        return ways[n - 1];
    }
};
