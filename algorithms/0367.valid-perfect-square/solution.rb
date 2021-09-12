# @param {Integer} num
# @return {Boolean}
def is_perfect_square(num)
  r = num == 1 ? 1 : num / 2
  while r * r > num
    # x0*2 = (x0**2 -k) / (x0 -x1)
    # x1 = x0 - (x0**2 - k) / x0*2
    #    = (x0**2 + k) / x0*2
    #    = (x0 + k/x0) / 2
    r = (r + num / r) / 2
  end
  r * r == num
end
