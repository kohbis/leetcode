# @param {Integer} n
# @return {Integer[]}
def even_odd_bit(n)
  even, odd = 0, 0
  n.to_s(2).chars.reverse.each_with_index do |c, i|
    if c == '1'
      if i % 2 == 0
        even += 1
      else
        odd += 1
      end
    end
  end
  [even, odd]
end
