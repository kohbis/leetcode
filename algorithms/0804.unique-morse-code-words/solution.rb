# @param {String[]} words
# @return {Integer}
def unique_morse_representations(words)
  morses = [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."]
  alph = ("a".."z").to_a
  
  words.map { |w| w.chars.map { |c| morses[alph.find_index(c)] }.join() }.uniq.count
end
