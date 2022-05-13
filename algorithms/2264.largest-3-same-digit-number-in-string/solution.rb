# @param {String} num
# @return {String}
def largest_good_integer(num)
  9.downto(0) do |n|
    p = n.to_s * 3
    return p if num.include?(p)
  end

  ''
end
