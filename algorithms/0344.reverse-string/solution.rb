# @param {Character[]} s
# @return {Void} Do not return anything, modify s in-place instead.
def reverse_string(s)
  # It's lazy
  # s.reverse!

  # in-place
  i, j = 0, s.size - 1
  while i < j
    s[i], s[j] = s[j], s[i]
    i += 1
    j -= 1
  end
  s
end
