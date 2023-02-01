# @param {String} str1
# @param {String} str2
# @return {String}
def gcd_of_strings(str1, str2)
  return str1 if str1 == str2
  return gcd_of_strings(str2, str1) if str1.length < str2.length
  return gcd_of_strings(str1[str2.length..], str2) if str1.start_with?(str2)
  ''
end
