# @param {Integer} num
# @return {Integer}
def add_digits(num)
  ## one-line
  # num = num.to_s.split("").map(&:to_i).sum while num >= 10

  while num >= 10
    sum = 0
    while num != 0
      sum += num % 10
      num /= 10
    end
    num = sum
  end

  num
end
