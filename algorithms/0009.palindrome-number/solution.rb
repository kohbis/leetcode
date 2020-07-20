# @param {Integer} x
# @return {Boolean}
def is_palindrome(x)
  # x.to_s == x.to_s.reverse

  s = x.to_s
  half = s.length / 2
  s[0..half] === s.reverse[0..half]
end
