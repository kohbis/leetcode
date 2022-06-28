# @param {String} s
# @return {Integer}
def min_operations(s)
  count1 = count2 = 0

  (0...s.length).each do |i|
    if i % 2 == 0
      if s[i] == '0'
        count1 += 1
      else
        # s[i] == '1'
        count2 += 1
      end
    else
      if s[i] == '1'
        count1 += 1
      else
        # s[i] == '0'
        count2 += 1
      end
    end
  end

  [count1, count2].min
end
