# @param {Integer[]} arr
# @return {Boolean}
def unique_occurrences(arr)
  count = Hash.new(0)
  arr.each { |i| count[i] += 1 }
  count.values.size == count.values.uniq.size
end
