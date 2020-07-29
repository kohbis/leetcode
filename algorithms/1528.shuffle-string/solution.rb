# @param {String} s
# @param {Integer[]} indices
# @return {String}
def restore_string(s, indices)
  res = Array.new(s.length, "")
  indices.each_with_index {|n, idx| res[n] = s[idx].chr }
  res.join
end

