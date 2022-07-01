function maximumUnits(boxTypes: number[][], truckSize: number): number {
  const sortedBoxTypes = boxTypes.sort((a, b) => b[1] - a[1]);
  let res = 0;

  for (const v of sortedBoxTypes) {
    const [num, unit] = v;
    if (truckSize < num) {
      res += truckSize * unit;
      break;
    } else {
      res += num * unit;
      truckSize -= num;
    }
  }

  return res;
}
