# @param {Integer[]} nums1
# @param {Integer[]} nums2
# @return {Integer[]}
def intersection(nums1, nums2)
  # nums1 & nums2

  res = []
  nums1.uniq.each do |n|
    if nums2.include?(n)
      res << n
      nums2.delete(n)
    end
  end
  res
end
