#include <vector>

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

        if (max > 0) {
            return max;
        } else {
            return -1;
        }
    }
};
