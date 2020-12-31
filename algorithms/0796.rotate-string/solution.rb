# @param {String} a
# @param {String} b
# @return {Boolean}
def rotate_string(a, b)
  return true if a == "" && b == ""

  for i in 1..a.size-1
    return true if b == a[-i..] + a[0..-i-1]
  end
  false
end

