# @param {Integer[]} a
# @return {Integer[]}
def sorted_squares(a)
  a.map { |i| i ** 2 }.sort
end
