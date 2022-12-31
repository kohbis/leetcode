# @param {Integer} n
# @return {Integer}
def pivot_integer(n)
  for i in 1..n
    return i if (1..i).sum == (i..n).sum
  end
  -1
end
