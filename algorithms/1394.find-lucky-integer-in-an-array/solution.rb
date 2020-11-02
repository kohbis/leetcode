# @param {Integer[]} arr
# @return {Integer}
def find_lucky(arr)
  arr.uniq.map {|num| num == arr.count(num) ? num : -1 }.max
end
