# @param {String} s
# @param {Integer} k
# @return {String}
def reverse_str(s, k)
  for i in (0...s.length).step(2 * k)
    s[i...i + k] = s[i...i + k].reverse
  end

  s
end
