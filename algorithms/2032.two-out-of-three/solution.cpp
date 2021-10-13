class Solution {
  public:
    vector<int> twoOutOfThree(vector<int>& nums1, vector<int>& nums2, vector<int>& nums3) {
        vector<int> res;

        for (int i = 1; i <= 100; ++i) {
            int count = 0;

            bool find1 = find(nums1.begin(), nums1.end(), i) != nums1.end();
            bool find2 = find(nums2.begin(), nums2.end(), i) != nums2.end();
            bool find3 = find(nums3.begin(), nums3.end(), i) != nums3.end();

            if (find1) {
                count++;
            }
            if (find2) {
                count++;
            }
            if (find3) {
                count++;
            }

            if (count >= 2) {
                res.push_back(i);
            }
        }

        return res;
    }
};
