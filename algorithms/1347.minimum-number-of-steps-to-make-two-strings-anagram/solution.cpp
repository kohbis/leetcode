#include <string>

using namespace std;

class Solution {
public:
    int minSteps(string s, string t) {
        int count[26] = {0};
        int n = s.size();
        for (int i = 0; i < n; i++) {
            count[s[i] - 'a']++;
            count[t[i] - 'a']--;
        }

        int res = 0;
        for (int i = 0; i < 26; i++) {
            if (0 < count[i]) {
                res += count[i];
            }
        }

        return res;
    }
};
