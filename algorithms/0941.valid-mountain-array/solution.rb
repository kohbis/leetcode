# @param {Integer[]} a
# @return {Boolean}
def valid_mountain_array(a)
  return false unless a.size >= 3
  return false unless a.count(a.max) == 1

  top = a.find_index(a.max)
  left, right = a[...top], a[top+1..]

  return false if left.empty? || right.empty?

  (left == left.sort.uniq) && (right == right.sort.uniq.reverse)
end
