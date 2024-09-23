class Solution {
public:
    int generateKey(int num1, int num2, int num3) {
        int div = 1000;
        int key = 0;
        for (int i = 0; i < 4; i++) {
            int digit = min({num1 / div, num2 / div, num3 / div});
            key = key * 10 + digit;

            num1 %= div;
            num2 %= div;
            num3 %= div;

            div /= 10;
        }
        return key;
    }
};
