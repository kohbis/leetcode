#include <vector>

using namespace std;

class Solution {
public:
    int threeSumClosest(vector<int>& nums, int target) {
        int size = nums.size();
        sort(nums.begin(), nums.end());

        int closest = 0;
        int min_diff = INT_MAX;
        for (int i = 0; i < size; i++) {
            int j = i + 1;
            int k = size - 1;

            while (j < k) {
                int sum = nums[i] + nums[j] + nums[k];
                if (sum == target) {
                    return target;
                } else if (sum > target) {
                    k--;
                } else {
                    j++;
                }

                if (abs(sum - target) < min_diff) {
                    closest = sum;
                    min_diff = abs(sum - target);
                }
            }
        }

        return closest;
    }
};
