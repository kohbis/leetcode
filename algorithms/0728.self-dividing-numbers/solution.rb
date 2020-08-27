# @param {Integer} left
# @param {Integer} right
# @return {Integer[]}
def self_dividing_numbers(left, right)
  res = []

  (left..right).each do |n|
    next res << n if n < 10

    divisor = n.to_s.chars.map(&:to_i)
    self_dividing = divisor.each do |d|
      if d != 0 && n % d == 0
        true
      else
        break false
      end
    end
    res << n if self_dividing
  end

  res
end

