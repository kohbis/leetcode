# @param {Integer} n
# @return {Integer}
def total_money(n)
  total = 0
  week, extra_day = n / 7, n % 7

  # sum of first-week (1..7).to_a.sum = 28
  total += 28 * week
  # 0(0) + 1(1) + 2(3) + 3(6) + 4(10) ...
  total += week * (week - 1) / 2 * 7
  # Monday of Nth week $money is $N
  total += week * extra_day
  # 0(0) + 1(1) + 2(3) + 3(6) + 4(10) ...
  total += extra_day * (extra_day + 1) / 2

  total
end

