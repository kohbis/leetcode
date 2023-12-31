#include <map>
#include <string>
#include <unordered_map>

using namespace std;

class Solution {
public:
    int maxLengthBetweenEqualCharacters(string s) {
        int longest = -1;
        unordered_map<char, int> map;

        for (int i = 0; i < s.length(); i++) {
            char c = s[i];
            if (map.count(c)) {
                longest = max(longest, i - map[c] - 1);
            } else {
                map[c] = i;
            }
        }

        return longest;
    }
};
