#include <vector>

using namespace std;

class Solution {
public:
    int minOperations(vector<int>& nums) {
        unordered_map<int, int> m;
        for (int n : nums) {
            m[n]++;
        }

        int res = 0;
        for (auto const& pair : m) {
            int v = pair.second;
            if (v == 1) {
                return -1;
            }
            res += v / 3;
            if (v % 3 != 0) {
                res++;
            }
        }

        return res;
    }
};
