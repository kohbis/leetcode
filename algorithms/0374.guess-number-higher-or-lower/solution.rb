# The guess API is already defined for you.
# @param num, your guess
# @return -1 if my number is lower, 1 if my number is higher, otherwise return 0
# def guess(num)

def guessNumber(n)
  left, right = 0, n

  loop {
    mid = left  + (right - left) / 2

    case guess(mid)
    when 0  then return mid
    when -1 then right = mid - 1
    when 1  then left = mid + 1
    end
  }
end
