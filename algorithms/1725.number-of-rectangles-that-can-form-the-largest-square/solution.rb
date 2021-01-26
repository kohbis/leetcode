# @param {Integer[][]} rectangles
# @return {Integer}
def count_good_rectangles(rectangles)
  max_len = 0
  max_len_count = 0

  rectangles.each do |r|
    min_len = r.min

    if min_len > max_len
      max_len = min_len
      max_len_count = 1
    elsif min_len == max_len
      max_len_count += 1
    end
  end

  max_len_count
end
