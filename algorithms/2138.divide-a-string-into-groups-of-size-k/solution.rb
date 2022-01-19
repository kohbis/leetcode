# @param {String} s
# @param {Integer} k
# @param {Character} fill
# @return {String[]}
def divide_string(s, k, fill)
  res, c = [], s.chars
  res << c.shift(k).join.ljust(k, fill) while c.size > 0
  res
end
