class Solution {
  public:
    int arrangeCoins(int n) {
        int row = 1;

        while (0 < n) {
            row++;
            n -= row;
        }

        return row - 1;
    }
};
