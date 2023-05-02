function arraySign(nums: number[]): number {
  let neg = 0;
  for (const n of nums) {
    if (n == 0) {
      return 0;
    }
    if (n < 0) {
      neg++;
    }
  }
  return neg % 2 == 0 ? 1 : -1;
}
