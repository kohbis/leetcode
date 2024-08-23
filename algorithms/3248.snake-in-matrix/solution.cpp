#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    int finalPositionOfSnake(int n, vector<string>& commands) {
        int i = 0;
        int j = 0;
        for (auto command : commands) {
            if (command == "UP") {
                i--;
            } else if (command == "DOWN") {
                i++;
            } else if (command == "LEFT") {
                j--;
            } else if (command == "RIGHT") {
                j++;
            }
            i = (i + n) % n;
            j = (j + n) % n;
        }
        return i * n + j;
    }
};
