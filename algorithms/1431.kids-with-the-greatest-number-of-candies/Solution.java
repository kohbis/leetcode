class Solution {
    public List<Boolean> kidsWithCandies(int[] candies, int extraCandies) {
        int maxCandies = 0;
        List<Boolean> res = new ArrayList<Boolean>(candies.length);

        for (int candy : candies) {
            if (candy > maxCandies) {
                maxCandies = candy;
            }
        }

        for (int candy : candies) {
            res.add(candy + extraCandies >= maxCandies);
        }

        return res;
    }
}
