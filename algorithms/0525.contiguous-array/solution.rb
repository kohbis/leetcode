# @param {Integer[]} nums
# @return {Integer}
def find_max_length(nums)
  res = 0
  cnt, h = 0, { 0 => -1 }
  nums.each_with_index do |num, idx|
    cnt += num == 0 ? -1 : 1
    if h.has_key?(cnt)
      res = [res, idx - h[cnt]].max
    else
      h[cnt] = idx
    end
  end
  res
end
