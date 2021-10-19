#include <vector>

using namespace std;

class Solution {
  public:
    vector<int> sortArray(vector<int>& nums) { return mergeSort(nums); }

    vector<int> mergeSort(vector<int>& vec) {
        if (vec.size() <= 1) {
            return vec;
        }

        auto begin = vec.begin();
        auto end = vec.end();
        auto pivot = begin + vec.size() / 2;

        vector<int> v1(begin, pivot);
        vector<int> v2(pivot, end);

        vector<int> left = mergeSort(v1);
        vector<int> right = mergeSort(v2);

        vector<int> res;
        int l = 0;
        int r = 0;

        while (l < left.size() && r < right.size()) {
            if (left[l] < right[r]) {
                res.push_back(left[l]);
                l += 1;
            } else {
                res.push_back(right[r]);
                r += 1;
            }
        }

        while (l < left.size()) {
            res.push_back(left[l]);
            l += 1;
        }

        while (r < right.size()) {
            res.push_back(right[r]);
            r += 1;
        }

        return res;
    }
};
