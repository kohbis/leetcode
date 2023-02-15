# @param {Integer[]} num
# @param {Integer} k
# @return {Integer[]}
def add_to_array_form(num, k)
  (num.map(&:to_s).join.to_i + k).digits.reverse
end
