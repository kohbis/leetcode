# @param {String} key
# @param {String} message
# @return {String}
def decode_message(key, message)
  alphabet = ('a'..'z').to_a
  table = key.chars.each_with_object({}) do |c, hash|
    next if c == ' ' || hash.key?(c)

    hash[c] = alphabet.shift
  end

  message.chars.inject('') do |result, c|
    result << (c == ' ' ? c : table[c])
  end
end
