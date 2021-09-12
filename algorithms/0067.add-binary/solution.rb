# @param {String} a
# @param {String} b
# @return {String}
def add_binary(a, b)
  i_a, i_b, carry = a.length, b.length, 0
  answer = ""

  while i_a > 0 || i_b > 0 || carry > 0
    if i_a > 0
      i_a -= 1
      carry += a[i_a].to_i
    end
    if i_b > 0
      i_b -= 1
      carry += b[i_b].to_i
    end

    answer.prepend((carry % 2).to_s)
    carry /= 2
  end

  answer
end
