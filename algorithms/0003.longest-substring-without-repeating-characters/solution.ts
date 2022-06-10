function lengthOfLongestSubstring(s: string): number {
  let curr = [];
  let longest = 0;

  for (const c of s) {
    const idx = curr.indexOf(c);
    if (idx >= 0) {
      curr = curr.slice(idx + 1);
    }

    curr.push(c);
    if (curr.length > longest) {
      longest = curr.length;
    }
  }

  return longest;
}
