function detectCapitalUse(word: string): boolean {
  const chars: string[] = word.split("");
  let count = 0;

  for (const c of chars) {
    if (c == c.toUpperCase()) {
      count++;
    }
  }
  return (
    count == 0 ||
    count == word.length ||
    (count == 1 && chars[0] == chars[0].toUpperCase())
  );
}
