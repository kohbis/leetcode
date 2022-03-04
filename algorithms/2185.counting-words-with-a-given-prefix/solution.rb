# @param {String[]} words
# @param {String} pref
# @return {Integer}
def prefix_count(words, pref)
  words.count { |w| w.start_with?(pref) }
end
