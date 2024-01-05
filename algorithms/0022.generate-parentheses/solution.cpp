#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    vector<string> generateParenthesis(int n) {
        vector<string> res;
        backtrace(n, n, "", res);
        return res;
    }

private:
    void backtrace(int left, int right, string current, vector<string>& res) {
        if (left == 0 && right == 0) {
            res.push_back(current);
            return;
        }
        if (0 < left) {
            backtrace(left - 1, right, current + "(", res);
        }
        if (left < right) {
            backtrace(left, right - 1, current + ")", res);
        }
    }
};
