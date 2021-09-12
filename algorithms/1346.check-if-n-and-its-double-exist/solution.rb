# @param {Integer[]} arr
# @return {Boolean}
def check_if_exist(arr)
  hash = {}
  arr.each do |i|
    return true if hash[i * 2]
    return true if i % 2 == 0 && hash[i / 2]

    hash[i] = true
  end

  # default
  false
end
