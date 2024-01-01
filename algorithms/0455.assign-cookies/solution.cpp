#include <vector>

using namespace std;

class Solution {
public:
    int findContentChildren(vector<int>& g, vector<int>& s) {
        sort(begin(g), end(g));
        sort(begin(s), end(s));

        int res = 0;
        int child_idx = 0;
        int cookie_idx = 0;

        while (child_idx < g.size() && cookie_idx < s.size()) {
            if (g[child_idx] <= s[cookie_idx]) {
                res += 1;
                child_idx += 1;
            }
            cookie_idx += 1;
        }
        return res;
    }
};
