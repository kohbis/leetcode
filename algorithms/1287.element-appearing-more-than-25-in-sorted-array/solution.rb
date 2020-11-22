# @param {Integer[]} arr
# @return {Integer}
def find_special_integer(arr)
  quarter = arr.size * 0.25
  arr.uniq.each do |n|
    return n if arr.count(n) > quarter
  end
end
