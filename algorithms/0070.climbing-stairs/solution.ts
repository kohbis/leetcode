function climbStairs(n: number): number {
  const a = new Array<number>(n).fill(0);
  for (let i = 0; i < n; i++) {
    switch (i) {
      case 0:
        a[i] = 1;
        break;
      case 1:
        a[i] = 2;
        break;
      default:
        a[i] = a[i - 2] + a[i - 1];
    }
  }
  return a[n - 1];
}
