# @param {Integer[]} nums
# @return {Integer[]}
def sort_array(nums)
  merge_sort(nums)
end

def merge_sort(arr)
  return arr if arr.size <= 1

  pivot = arr.size / 2

  left =  merge_sort(arr[0..pivot - 1])
  right = merge_sort(arr[pivot..-1])

  res = []

  while left.size > 0 && right.size > 0
    if left[0] < right[0]
      res << left.shift
    else
      res << right.shift
    end
  end

  while left.size > 0
    res << left.shift
  end

  while right.size > 0
    res << right.shift
  end

  res
end
