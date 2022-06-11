function minMaxGame(nums: number[]): number {
  if (nums.length == 1) {
    return nums[0];
  }

  let mid = nums.length / 2;
  while (mid > 0) {
    for (let i = 0; i < mid; i++) {
      if (i % 2 == 0) {
        nums[i] = Math.min(nums[i * 2], nums[i * 2 + 1]);
      } else {
        nums[i] = Math.max(nums[i * 2], nums[i * 2 + 1]);
      }
    }
    mid /= 2;
  }

  return nums[0];
}
