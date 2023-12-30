#include <vector>

using namespace std;

class Solution {
public:
    int countKDifference(vector<int>& nums, int k) {
        int count = 0;
        size_t size = nums.size();

        for (int i = 0; i < (size - 1); i++) {
            for (int j = (i + 1); j < size; j++) {
                if (abs(nums[i] - nums[j]) == k) {
                    count++;
                }
            }
        }

        return count;
    }
};
