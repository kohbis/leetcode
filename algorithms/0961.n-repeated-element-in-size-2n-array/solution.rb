# @param {Integer[]} a
# @return {Integer}
def repeated_n_times(a)
  a.group_by(&:itself).sort_by {|k, v| v.count }.last[0]
end

