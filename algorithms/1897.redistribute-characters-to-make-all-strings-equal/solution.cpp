#include <string>
#include <vector>

using namespace std;

class Solution {
  public:
    bool makeEqual(vector<string>& words) {
        if (words.size() == 1) {
            return true;
        }

        vector<int> counter(26, 0);
        for (const string& word : words) {
            for (char c : word) {
                counter[c - 'a']++;
            }
        }

        for (int i : counter) {
            if (i % words.size() != 0) {
                return false;
            }
        }

        return true;
    }
};
