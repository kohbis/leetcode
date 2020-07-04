# @param {Integer[]} group_sizes
# @return {Integer[][]}
def group_the_people(group_sizes)
  group_sizes.each_with_index.group_by(&:first).flat_map { |k, v| v.map(&:last).each_slice(k).to_a }
end
