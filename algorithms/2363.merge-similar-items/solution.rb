# @param {Integer[][]} items1
# @param {Integer[][]} items2
# @return {Integer[][]}
def merge_similar_items(items1, items2)
  count = Hash.new(0)
  items1.concat(items2).each do |item|
    count[item[0]] += item[1]
  end
  count.sort
end
