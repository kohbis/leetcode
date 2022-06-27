function minPartitions(n: string): number {
  let max = -1;
  for (let c of n) {
    if (max < Number(c)) {
      max = Number(c);
    }
  }
  return max;
}
