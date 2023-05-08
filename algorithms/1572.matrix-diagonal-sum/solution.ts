function diagonalSum(mat: number[][]): number {
  const len = mat.length;
  let i = Math.floor(len / 2);
  let sum = 0;

  if (len % 2 == 1) {
    sum += mat[i][i];
  }

  while (i > 0) {
    i--;
    const j = len - i - 1;
    sum += mat[i][i] + mat[i][j] + mat[j][i] + mat[j][j];
  }

  return sum;
}
