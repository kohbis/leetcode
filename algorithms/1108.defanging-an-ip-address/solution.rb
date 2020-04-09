# @param {String} address
# @return {String}
def defang_i_paddr(address)
  # address.gsub!(".", "[.]")

  address.chars.inject('') do |res, char|
    res += char == '.' ? '[.]' : char
  end
end
