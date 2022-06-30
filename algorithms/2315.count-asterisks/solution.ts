function countAsterisks(s: string): number {
  let count = 0;
  let inside = true;

  for (const c of s) {
    switch (c) {
      case "|":
        inside = !inside;
        break;
      case "*":
        if (inside) {
          count++;
        }
        break;
    }
  }

  return count;
}
