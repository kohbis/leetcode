class Solution {
public:
    int countPrefixSuffixPairs(vector<string>& words) {
        int count = 0;
        int n = words.size();
        for (int i = 0; i < n - 1; i++) {
            for (int j = i + 1; j < n; j++) {
                if (isPrefixAndSuffix(words[i], words[j])) {
                    count++;
                }
            }
        }
        return count;
    }

    bool isPrefixAndSuffix(string& a, string& b) {
        int n = a.size();
        int m = b.size();
        if (n > m) {
            return false;
        }
        string prefix = b.substr(0, n);
        string suffix = b.substr(m - n);
        return prefix == a && suffix == a;
    }
};
