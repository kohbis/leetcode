# @param {Integer[]} arr
# @return {Boolean}
def unique_occurrences(arr)
  counts = Hash.new(0)
  arr.each { |i| counts[i] += 1 }
  counts.values.size == counts.values.uniq.size
end
