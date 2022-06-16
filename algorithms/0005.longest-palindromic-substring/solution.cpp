class Solution {
    bool isPalindrome(string& s, int left, int right) {
        while (left < right) {
            if (s[left] != s[right]) {
                return false;
            }

            left++;
            right--;
        }
        return true;
    }

  public:
    string longestPalindrome(string s) {
        int len = s.length();
        int longest = 0;
        int left = 0;
        int right = 0;

        for (int i = 0; i < len - 1; i++) {
            for (int j = i + 1; j < len; j++) {
                if (longest > j - i) {
                    continue;
                }

                if (isPalindrome(s, i, j)) {
                    longest = j - i;
                    left = i;
                    right = j;
                }
            }
        }

        return s.substr(left, right - left + 1);
    }
};
