# @param {Integer[]} a
# @return {Boolean}
def valid_mountain_array(a)
  return false if a.size < 3

  i = 1
  while i < a.size
    break if a[i] <= a[i-1]
    i += 1
  end
  
  return false if i == a.size || i == 1
  
  while i < a.size
    return false if a[i] >= a[i-1]
    i += 1
  end

  true
end


# # @param {Integer[]} a
# # @return {Boolean}
# def valid_mountain_array(a)
#   return false unless a.size >= 3
#   return false unless a.count(a.max) == 1

#   top = a.find_index(a.max)
#   left, right = a[...top], a[top+1..]

#   return false if left.empty? || right.empty?

#   (left == left.sort.uniq) && (right == right.sort.uniq.reverse)
# end
