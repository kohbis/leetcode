function countOdds(low: number, high: number): number {
  const diff = high - low;
  let sum = Math.floor(diff / 2);
  if (diff % 2 == 1 || low % 2 == 1) {
    sum++;
  }
  return sum;
}
