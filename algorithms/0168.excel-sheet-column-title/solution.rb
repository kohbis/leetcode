ALPH = ("A".."Z").to_a.freeze

# @param {Integer} n
# @return {String}
def convert_to_title(n)
  res = ""
  while n > 0
    n -= 1
    res.prepend(ALPH[n % 26])
    n /= 26
  end
  res
end
