#include <string>
#include <unordered_set>

using namespace std;

class Solution {
public:
    bool halvesAreAlike(string s) {
        unordered_set<char> vowels = {'a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'};

        int length = s.length();
        int mid = length / 2;
        int aVowels = 0;
        int bVowels = 0;
        for (int i = 0; i < length; i++) {
            if (vowels.count(s[i])) {
                if (i < mid) {
                    aVowels++;
                } else {
                    bVowels++;
                }
            }
        }

        return aVowels == bVowels;
    }
};
