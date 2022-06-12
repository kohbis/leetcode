# @param {String[]} strs
# @return {String}
def longest_common_prefix(strs)
  min_len = strs.map(&:length).min

  (0...min_len).each do |i|
    c = strs[0][i]

    strs.each do |s|
      return s[0...i] if s[i] != c
    end
  end

  strs[0][0...min_len]
end
