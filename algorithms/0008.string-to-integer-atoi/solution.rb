# @param {String} s
# @return {Integer}
def my_atoi(s)
  str = s.strip.split(' ').first
  return 0 if str == nil || s.length == 0

  sign, total, start = 1, 0, 0
  # check signs
  if str[0] == '+' || str[0] == '-'
    sign = str[0] == '+' ? 1 : -1
    start += 1
  end

  digits = ('0'..'9').to_a
  (start...str.length).each do |i|
    digit = if digits.include?(str[i])
        Integer(str[i])
      else
        return sign * total
      end

    total = total * 10 + digit
    return 2_147_483_647 if sign * total > 2_147_483_647
    return -2_147_483_648 if sign * total < -2_147_483_648
  end

  sign * total
end
