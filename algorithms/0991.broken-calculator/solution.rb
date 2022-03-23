# @param {Integer} start_value
# @param {Integer} target
# @return {Integer}
def broken_calc(start_value, target)
  count = 0

  while start_value < target
    count += 1

    if target % 2 == 0
      target /= 2
    else
      target += 1
    end
  end

  count + start_value - target
end
