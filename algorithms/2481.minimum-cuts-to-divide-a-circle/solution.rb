# @param {Integer} n
# @return {Integer}
def number_of_cuts(n)
  return 0 if n == 1
  n % 2 == 0 ? n / 2 : n
end
