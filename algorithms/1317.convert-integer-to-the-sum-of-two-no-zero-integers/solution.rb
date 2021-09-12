# @param {Integer} n
# @return {Integer[]}
def get_no_zero_integers(n)
  (1...n).each do |i|
    if no_zero?(i) && no_zero?(n - i)
      return [i, n - i]
    end
  end
end

def no_zero?(num)
  while num > 0
    return false if num % 10 == 0

    num /= 10
  end

  true
end
