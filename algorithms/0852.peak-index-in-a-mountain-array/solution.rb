# @param {Integer[]} arr
# @return {Integer}
def peak_index_in_mountain_array(arr)
  for i in 1...arr.size-1
    return i if arr[i-1] < arr[i] && arr[i+1] < arr[i]
  end
end
