#include <vector>

using namespace std;

class Solution {
public:
    double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {
        vector<int> nums = merge(nums1, nums2);
        int mid = nums.size() / 2;

        if (nums.size() % 2 == 0) {
            int sum = nums[mid - 1] + nums[mid];
            return (double)sum / 2.0;
        } else {
            return (double)nums[mid];
        }
    }

    vector<int> merge(vector<int>& nums1, vector<int>& nums2) {
        vector<int> nums;

        int i = 0;
        int j = 0;

        while (i < nums1.size() && j < nums2.size()) {
            if (nums1[i] < nums2[j]) {
                nums.push_back(nums1[i]);
                i += 1;
            } else {
                nums.push_back(nums2[j]);
                j += 1;
            }
        }

        while (i < nums1.size()) {
            nums.push_back(nums1[i]);
            i += 1;
        }

        while (j < nums2.size()) {
            nums.push_back(nums2[j]);
            j += 1;
        }

        return nums;
    }
};
