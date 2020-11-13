# @param {Integer[]} arr
# @return {Integer[]}
def array_rank_transform(arr)
  rank = {}
  arr.sort.uniq.each_with_index do |score, idx|
    rank[score] = idx + 1
  end
  arr.map {|score| rank[score]}
end
