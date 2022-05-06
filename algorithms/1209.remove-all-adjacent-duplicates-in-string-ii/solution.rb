# @param {String} s
# @param {Integer} k
# @return {String}
def remove_duplicates(s, k)
  stack = [] # [[char, count]]

  s.each_char do |c|
    if !stack.size.empty? && stack[-1][0] == c
      stack.push([c, stack[-1][1] + 1])

      stack.pop(k) if stack[-1][1] == k
    else
      stack.push([c, 1])
    end
  end

  stack.map { |st| st[0] }.join
end
