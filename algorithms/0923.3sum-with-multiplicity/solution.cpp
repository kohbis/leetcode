#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
  public:
    int threeSumMulti(vector<int>& arr, int target) {
        int res = 0;
        int mod = 1000000007;
        unordered_map<int, int> m;

        for (int i = 0; i < arr.size(); i++) {
            for (int j = i + 1; j < arr.size(); j++) {
                int remain = target - arr[i] - arr[j];
                res += m[remain];
            }

            res %= mod;
            m[arr[i]]++;
        }

        return res % mod;
    }
};
