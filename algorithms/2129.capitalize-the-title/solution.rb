# @param {String} title
# @return {String}
def capitalize_title(title)
  words = []

  title.split.each do |word|
    tmp = word.downcase
    if word.size > 2
      words << tmp[0].upcase + tmp[1..]
    else
      words << tmp
    end
  end

  words.join(' ')
end
