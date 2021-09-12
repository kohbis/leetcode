# @param {String} s
# @return {Boolean}
def is_palindrome(s)
  words = s.downcase.gsub(/[^a-z0-9]/, "")
  words == words.reverse
end
