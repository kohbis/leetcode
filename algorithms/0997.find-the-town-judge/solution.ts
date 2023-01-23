function findJudge(n: number, trust: number[][]): number {
  const peaple = new Array(n).fill(0);
  for (const t of trust) {
    peaple[t[0] - 1]--;
    peaple[t[1] - 1]++;
  }
  const judge = peaple.indexOf(n - 1);
  return judge === -1 ? -1 : judge + 1;
}
