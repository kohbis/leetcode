# @param {Integer} num
# @return {Integer}
def count_digits(num)
  num.digits.select { |x| num % x == 0 }.size
end
