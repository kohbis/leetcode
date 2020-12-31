# @param {Integer} num
# @return {String}
def convert_to_base7(num)
  ## Lazy solution
  # num.to_s(7)

  return "0" if num == 0

  res = []
  n = num.abs

  while n > 0
    rem = n % 7
    n /= 7
    res.unshift(rem.to_s)
  end

  res.unshift("-") if num < 0
  res.join
end

