#include <vector>

using namespace std;

class Solution {
public:
    int timeRequiredToBuy(vector<int>& tickets, int k) {
        int seconds = 0;
        int n = tickets.size();

        while (true) {
            for (int i = 0; i < n; i++) {
                if (tickets[i] > 0) {
                    seconds++;
                    tickets[i]--;

                    if (k == i && tickets[i] == 0) {
                        return seconds;
                    }
                }
            }
        }

        return seconds;
    }
};
