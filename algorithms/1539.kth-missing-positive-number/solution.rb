# @param {Integer[]} arr
# @param {Integer} k
# @return {Integer}
def find_kth_positive(arr, k)
  ((1..(arr.size + k)).to_a - arr)[k - 1]
end
