# @param {Integer} n
# @return {Integer}
def number_of_matches(n)
  count = 0

  while n > 1
    if n % 2 == 0
      n = n / 2
      count += n
    else
      n = (n - 1) / 2
      count += n + 1
    end
  end

  count
end
