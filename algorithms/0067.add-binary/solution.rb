# @param {String} a
# @param {String} b
# @return {String}
def add_binary(a, b)
  i, j, carry = a.length, b.length, 0
  answer = ''

  while i > 0 || j > 0 || carry > 0
    if i > 0
      i -= 1
      carry += a[i].to_i
    end
    if j > 0
      j -= 1
      carry += b[j].to_i
    end

    answer.prepend((carry % 2).to_s)
    carry /= 2
  end

  answer
end
