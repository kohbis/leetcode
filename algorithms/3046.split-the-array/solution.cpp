#include <vector>

using namespace std;

class Solution {
public:
    bool isPossibleToSplit(vector<int>& nums) {
        unordered_map<int, int> count;
        for (int num : nums) {
            count[num]++;
            if (count[num] > 2) {
                return false;
            }
        }
        return true;
    }
};
