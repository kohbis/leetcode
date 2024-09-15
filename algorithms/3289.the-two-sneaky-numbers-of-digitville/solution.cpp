#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> getSneakyNumbers(vector<int>& nums) {
        vector<int> sneakyNumbers;
        unordered_map<int, int> count;
        for (int num : nums) {
            count[num]++;
            if (count[num] > 1) {
                sneakyNumbers.push_back(num);
            }
            if (sneakyNumbers.size() == 2) {
                break;
            }
        }
        return sneakyNumbers;
    }
};
