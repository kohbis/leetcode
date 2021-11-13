# @param {String[]} arr
# @param {Integer} k
# @return {String}
def kth_distinct(arr, k)
  h = Hash.new([0, 0])

  arr.each_with_index do |c, i|
    if h.has_key?(c)
      h[c] = [h[c][0] + 1, i]
    else
      h[c] = [1, i]
    end
  end

  counts = h.select { |_, v| v[0] == 1 }.sort_by { |_, v| v[1] }
  counts.size >= k ? counts[k - 1][0] : ""
end
