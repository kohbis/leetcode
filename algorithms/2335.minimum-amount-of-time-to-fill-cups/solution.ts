function fillCups(amount: number[]): number {
  const max = Math.max(...amount);
  const sum = amount.reduce((sum, val) => sum + val, 0);

  return Math.max(max, sum / 2);
}
