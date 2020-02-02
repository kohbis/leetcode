# @param {String[]} strs
# @return {String}
def longest_common_prefix(strs)
  return "" if strs.length.zero?

  shortest = strs.min_by(&:length)
  return "" if shortest.empty?

  prefix = ""
  shortest.length.times do |i|
    curt = shortest[0..i]
    strs.each do |s|
      return prefix if s.index(curt) != 0
    end
    prefix = curt
  end
  
  prefix
end
