# @param {Integer[]} arr
# @return {Integer}
def sum_odd_length_subarrays(arr)
  count = 0
  i = 0
  while i < arr.size
    j = i
    while j < arr.size
      count += arr[i..j].sum
      j += 2
    end
    i += 1
  end

  count
end
