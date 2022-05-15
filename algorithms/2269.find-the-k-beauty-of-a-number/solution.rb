# @param {Integer} num
# @param {Integer} k
# @return {Integer}
def divisor_substrings(num, k)
  count = 0

  num.to_s.chars.each_cons(k) do |v|
    i = v.join.to_i
    next if i == 0
    count += 1 if num % i == 0
  end

  count
end
