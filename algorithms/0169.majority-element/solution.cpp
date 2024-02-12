#include <vector>

using namespace std;

class Solution {
public:
    int majorityElement(vector<int>& nums) {
        int res;
        unordered_map<int, int> m;

        for (int n : nums) {
            m[n]++;

            if (m[n] > m[res]) {
                res = n;
            }
        }

        return res;
    }
};
