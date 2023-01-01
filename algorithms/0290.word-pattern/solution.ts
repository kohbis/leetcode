function wordPattern(pattern: string, s: string): boolean {
  const chars: string[] = pattern.split("");
  const words: string[] = s.split(" ");

  if (words.length != chars.length) {
    return false;
  }

  let map = new Map<string, string>();
  for (let i = 0; i < chars.length; i++) {
    const c = chars[i];
    const word = words[i];

    if (map.has(c)) {
      if (map.get(c) != word) {
        return false;
      }
    } else {
      map.set(c, word);
    }
  }

  const values = Array.from(map.values());
  return values.length == new Set([...values]).size;
}
