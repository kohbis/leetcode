# @param {Integer} n
# @return {Boolean}
def is_happy(n)
  arr = []
  until arr.include?(n)
    sum = 0
    n.to_s.chars.map(&:to_i).each { |x| sum += x * x }
    return true if sum == 1
    arr << n
    n = sum
  end
  false
end
