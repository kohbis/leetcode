#include <vector>

using namespace std;

class Solution {
public:
    vector<vector<int>> findMatrix(vector<int>& nums) {
        vector<vector<int>> res;
        unordered_map<int, int> map;

        for (auto num : nums) {
            map[num] += 1;
        }

        while (!map.empty()) {
            vector<int> row;
            for (auto it = map.begin(); it != map.end();) {
                if (it->second > 0) {
                    row.push_back(it->first);
                    it->second--;
                }

                if (it->second == 0) {
                    it = map.erase(it);
                } else {
                    it++;
                }
            }

            res.push_back(row);
        }

        return res;
    }
};
