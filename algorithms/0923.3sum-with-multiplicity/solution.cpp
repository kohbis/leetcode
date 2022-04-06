#include <vector>
#include <unordered_map>

using namespace std;

class Solution {
  public:
    int threeSumMulti(vector<int>& arr, int target) {
        int ans = 0;
        int mod = 1000000007;
        unordered_map<int, int> m;

        for (int i = 0; i < arr.size(); i++) {
            for (int j = i + 1; j < arr.size(); j++) {
                int remain = target - arr[i] - arr[j];
                ans += m[remain];
            }

            ans %= mod;
            m[arr[i]]++;
        }

        return ans % mod;
    }
};
