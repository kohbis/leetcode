# @param {String[]} strs
# @return {Integer}
def maximum_value(strs)
  strs.map { |s| s.match?(/[a-z]/) ? s.size : s.to_i }.max
end
