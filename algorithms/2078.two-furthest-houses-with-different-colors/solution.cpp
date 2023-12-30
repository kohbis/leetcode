#include <vector>

using namespace std;

class Solution {
public:
    int maxDistance(vector<int>& colors) {
        int len = colors.size();

        int left = 0;
        while (colors[left] == colors[len - 1]) {
            left++;
        }

        int right = len - 1;
        while (colors[0] == colors[right]) {
            right--;
        }

        return max((len - 1 - left), right);
    }
};
