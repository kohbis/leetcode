function percentageLetter(s: string, letter: string): number {
  let count = 0;
  for (const c of s) {
    if (c == letter) {
      count++;
    }
  }
  return Math.floor((100 * count) / s.length);
}
