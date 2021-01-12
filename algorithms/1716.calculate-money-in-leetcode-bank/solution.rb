# @param {Integer} n
# @return {Integer}
def total_money(n)
  res = 0
  dollars, day = 1, 1

  while n > 0
    res += dollars

    dollars += 1
    day += 1

    if day > 7
      day = 1
      dollars -= 6
    end

    n -= 1
  end

  res
end
