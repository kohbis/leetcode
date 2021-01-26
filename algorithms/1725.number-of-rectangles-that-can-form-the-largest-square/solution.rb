# @param {Integer[][]} rectangles
# @return {Integer}
def count_good_rectangles(rectangles)
  hash = Hash.new(0)
  rectangles.each do |r|
    hash[r.min] += 1
  end
  hash.max[1]
end
