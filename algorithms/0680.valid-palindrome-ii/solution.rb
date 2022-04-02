# @param {String} s
# @return {Boolean}
def valid_palindrome(s)
  letters = s.chars
  left, right = 0, s.length - 1

  while left < right
    if letters[left] != letters[right]
      return is_palindrome(letters, left + 1, right) || is_palindrome(letters, left, right - 1)
    end

    left += 1
    right -= 1
  end

  true
end

def is_palindrome(letters, left, right)
  while left < right
    return false if letters[left] != letters[right]

    left += 1
    right -= 1
  end

  true
end
