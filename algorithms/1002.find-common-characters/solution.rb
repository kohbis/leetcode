# @param {String[]} a
# @return {String[]}
def common_chars(a)
  count_map = Hash.new(0)
  a[0].each_char { |c| count_map[c] += 1 }

  (1...a.size).each do |i|
    word = a[i]
    count_map.keys.each { |k| count_map[k] = [count_map[k], word.count(k)].min }
  end

  count_map.map { |k, v| k * v }.join.split('')
end
