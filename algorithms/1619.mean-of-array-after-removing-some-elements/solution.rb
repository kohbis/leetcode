# @param {Integer[]} arr
# @return {Float}
def trim_mean(arr)
  i = arr.size * 0.05
  arr.sort[i...-i].sum / (arr.size * 0.9)
end
