# @param {Integer[]} arr
# @return {Integer[]}
def replace_elements(arr)
  res = []
  (0...arr.size - 1).each do |i|
    res << arr[i + 1..-1].max
  end
  res << -1
end
