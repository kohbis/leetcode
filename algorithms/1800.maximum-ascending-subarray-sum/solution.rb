# @param {Integer[]} nums
# @return {Integer}
def max_ascending_sum(nums)
  subarrays = []

  tmp = []
  nums.each do |n|
    unless tmp.empty?
      unless tmp[-1] < n
        subarrays << tmp
        tmp = []
      end
    end
    tmp << n
  end
  subarrays << tmp

  subarrays.map(&:sum).max
end
