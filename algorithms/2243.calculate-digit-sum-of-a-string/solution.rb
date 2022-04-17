# @param {String} s
# @param {Integer} k
# @return {String}
def digit_sum(s, k)
  digits = s.chars
  while digits.size > k
    tmp = []
    digits.each_slice(k).map do |slice|
      tmp += slice.map(&:to_i).sum.to_s.chars
    end
    digits = tmp
  end

  digits.join
end
