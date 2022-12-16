function halvesAreAlike(s: string): boolean {
  const vowels = ["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"];
  const mid = s.length / 2;

  let aVowels = 0;
  let bVowels = 0;
  for (let i = 0; i < s.length; i++) {
    if (vowels.includes(s[i])) {
      if (i < mid) {
        aVowels++;
      } else {
        bVowels++;
      }
    }
  }

  return aVowels == bVowels;
}
