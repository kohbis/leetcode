#include <vector>

using namespace std;

class Solution {
public:
    int maximumDifference(vector<int>& nums) {
        int max;
        size_t size = nums.size();

        for (int i = 0; i < (size - 1); ++i) {
            for (int j = (i + 1); j < size; ++j) {
                if (nums[j] - nums[i] > max) {
                    max = nums[j] - nums[i];
                }
            }
        }

        if (0 < max) {
            return max;
        } else {
            return -1;
        }
    }
};
