# @param {Integer} n
# @return {String}
def generate_the_string(n)
  n.odd? ? ("s" * n) : (("s" * (n - 1)) + "t")
end
