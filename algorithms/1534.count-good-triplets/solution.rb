# @param {Integer[]} arr
# @param {Integer} a
# @param {Integer} b
# @param {Integer} c
# @return {Integer}
def count_good_triplets(arr, a, b, c)
  count = 0
  (0...arr.size-2).each do |i|
    (i+1...arr.size-1).each do |j|
      next unless (arr[i] - arr[j]).abs <= a
      (j+1...arr.size).each do |k|
        count += 1 if (arr[j] - arr[k]).abs <= b && (arr[i] - arr[k]).abs <= c
      end
    end
  end
  count
end

