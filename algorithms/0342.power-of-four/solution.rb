# @param {Integer} num
# @return {Boolean}
def is_power_of_four(num)
  # if num == 0
  #   false
  # elsif num == 1
  #   true
  # else
  #   num % 4 == 0 && is_power_of_four(num / 4)
  # end

  b = num.to_s(2)
  b[0] == '1' && b[1..-1].length.even? && b[1..-1].count('1') == 0
end
