function average(salary: number[]): number {
  let min = salary[0];
  let max = 0;
  let sum = 0;

  for (const s of salary) {
    if (s < min) {
      min = s;
    }
    if (s > max) {
      max = s;
    }
    sum += s;
  }

  return (sum - min - max) / (salary.length - 2);
}
