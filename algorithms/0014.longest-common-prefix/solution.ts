function longestCommonPrefix(strs: string[]): string {
  const min = Math.min.apply(
    0,
    strs.map((s) => s.length)
  );

  for (let i = 0; i < min; i++) {
    const c = strs[0].charAt(i);

    for (const s of strs) {
      if (s.charAt(i) !== c) {
        return s.substring(0, i);
      }
    }
  }

  return strs[0].substring(0, min);
}
