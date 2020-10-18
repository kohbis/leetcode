# @param {Integer[]} nums1
# @param {Integer[]} nums2
# @return {Integer[]}
def intersect(nums1, nums2)
  res = []
  long_list, short_list = [], []

  if nums1.size < nums2.size
    long_list = nums2
    short_list = nums1
  else
    long_list = nums1
    short_list = nums2
  end

  short_list.each do |n|
    idx = long_list.find_index(n)
    if idx
      res << n
      long_list.delete_at(idx)
    end
  end
  res
end
